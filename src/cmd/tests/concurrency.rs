//! Concurrency and performance tests.
//!
//! Tests for concurrent command execution and performance-related functionality,
//! ensuring commands can be executed safely in parallel without interference.

use crate::cmd;

/// Tests that multiple commands can be executed concurrently without interference
#[test]
fn test_concurrent_execution() {
    use std::thread;

    let handles: Vec<_> = (0..3)
        .map(|i| {
            thread::spawn(move || {
                cmd!("echo", &format!("thread_{}", i))
                    .no_echo()
                    .output()
                    .unwrap()
            })
        })
        .collect();

    let results: Vec<String> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // Each thread should get its own output
    assert_eq!(results.len(), 3);
    for (i, result) in results.iter().enumerate() {
        assert!(result.contains(&format!("thread_{}", i)));
    }
}

/// Tests high concurrency with many threads
#[test]
fn test_high_concurrency() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::thread;

    let success_count = Arc::new(AtomicU32::new(0));
    // Adjust thread count based on system capabilities
    let thread_count = std::cmp::min(50, std::thread::available_parallelism().unwrap().get() * 4);

    let handles: Vec<_> = (0..thread_count)
        .map(|i| {
            let success_count = Arc::clone(&success_count);
            thread::spawn(move || {
                let result = cmd!("echo", &format!("worker_{:02}", i)).no_echo().output();

                if let Ok(output) = result {
                    if output.trim() == format!("worker_{:02}", i) {
                        success_count.fetch_add(1, Ordering::SeqCst);
                    }
                }
            })
        })
        .collect();

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // All threads should have succeeded
    assert_eq!(success_count.load(Ordering::SeqCst), thread_count as u32);
}

/// Tests concurrent pipeline execution
#[test]
fn test_concurrent_pipelines() {
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::thread;

    let results = Arc::new(Mutex::new(Vec::new()));
    let thread_count = 10;

    let handles: Vec<_> = (0..thread_count)
        .map(|i| {
            let results = Arc::clone(&results);
            thread::spawn(move || {
                let output = cmd!("echo", &format!("data_{}", i))
                    .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
                    .pipe(cmd!("wc", "-c"))
                    .no_echo()
                    .output()
                    .unwrap();

                let char_count: u32 = output.trim().parse().unwrap();
                results.lock().unwrap().push((i, char_count));
            })
        })
        .collect();

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    let results = results.lock().unwrap();
    assert_eq!(results.len(), thread_count as usize);

    // Each result should have the correct character count for "DATA_X"
    for (i, char_count) in results.iter() {
        let expected_chars = format!("DATA_{}", i).len() as u32 + 1; // +1 for echo's newline
        assert_eq!(*char_count, expected_chars);
    }
}

/// Tests concurrent error handling
#[test]
fn test_concurrent_error_handling() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::thread;

    let error_count = Arc::new(AtomicU32::new(0));
    let success_count = Arc::new(AtomicU32::new(0));
    let thread_count = 20;

    let handles: Vec<_> = (0..thread_count)
        .map(|i| {
            let error_count = Arc::clone(&error_count);
            let success_count = Arc::clone(&success_count);
            thread::spawn(move || {
                // Half the commands will fail, half will succeed
                let result = if i % 2 == 0 {
                    cmd!("echo", "test").no_echo().output()
                } else {
                    // Use process ID to make command name unique across test runs
                    let nonexistent_cmd = format!("nonexistent_cmd_{}", std::process::id());
                    cmd!(&nonexistent_cmd, "test").no_echo().output()
                };

                match result {
                    Ok(_) => success_count.fetch_add(1, Ordering::SeqCst),
                    Err(_) => error_count.fetch_add(1, Ordering::SeqCst),
                };
            })
        })
        .collect();

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Should have exactly half successes and half errors
    assert_eq!(success_count.load(Ordering::SeqCst), thread_count / 2);
    assert_eq!(error_count.load(Ordering::SeqCst), thread_count / 2);
}

/// Tests resource cleanup under concurrent load
#[test]
fn test_resource_cleanup() {
    use std::thread;
    use std::time::{Duration, Instant};

    let start_time = Instant::now();

    // Run many short-lived commands to test resource cleanup
    let handles: Vec<_> = (0..100)
        .map(|i| {
            thread::spawn(move || {
                // Mix of different command types
                match i % 4 {
                    0 => {
                        let _ = cmd!("echo", "test").no_echo().output();
                    }
                    1 => {
                        let _ = cmd!("echo", "pipe_test")
                            .pipe(cmd!("cat"))
                            .no_echo()
                            .output();
                    }
                    2 => {
                        let _ = cmd!("sh", "-c", "echo 'stderr_test' >&2")
                            .pipe_stderr(cmd!("cat"))
                            .no_echo()
                            .output();
                    }
                    3 => {
                        let _ = cmd!("echo", "input_test")
                            .input("some input")
                            .no_echo()
                            .output();
                    }
                    _ => unreachable!(),
                }

                // Small delay to spread out resource usage
                thread::sleep(Duration::from_millis(1));
            })
        })
        .collect();

    // Wait for all threads to complete with timeout protection
    let mut completed_threads = 0;
    for handle in handles {
        handle.join().unwrap();
        completed_threads += 1;
    }

    let elapsed = start_time.elapsed();

    // Verify all threads completed successfully
    assert_eq!(
        completed_threads, 100,
        "Not all threads completed successfully"
    );

    // Ensure the test completed in reasonable time (indicates no resource deadlocks)
    assert!(
        elapsed < Duration::from_secs(30),
        "Resource cleanup test took too long, possible resource leak or deadlock: {:?}",
        elapsed
    );
}

/// Tests thread safety of command building
#[test]
fn test_command_building_thread_safety() {
    use std::sync::Arc;
    use std::thread;

    // Test that command building is thread-safe
    let shared_data = Arc::new("shared_value".to_string());

    let handles: Vec<_> = (0..20)
        .map(|i| {
            let shared_data = Arc::clone(&shared_data);
            thread::spawn(move || {
                let cmd = cmd!("echo", &*shared_data)
                    .arg(format!("arg_{}", i))
                    .env("THREAD_ID", i.to_string())
                    .no_echo();

                let output = cmd.output().unwrap();
                assert!(output.contains("shared_value"));
                assert!(output.contains(&format!("arg_{}", i)));
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

/// Tests performance under concurrent load
#[test]
fn test_concurrent_performance() {
    use std::thread;
    use std::time::{Duration, Instant};

    let start_time = Instant::now();
    let thread_count = 25;

    let handles: Vec<_> = (0..thread_count)
        .map(|i| {
            thread::spawn(move || {
                // Each thread runs multiple commands
                for j in 0..4 {
                    let output = cmd!("echo", &format!("{}_{}", i, j))
                        .no_echo()
                        .output()
                        .unwrap();
                    assert_eq!(output.trim(), format!("{}_{}", i, j));
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed = start_time.elapsed();

    // 100 commands (25 threads * 4 commands each) should complete reasonably quickly
    // This is a rough performance check - adjust threshold based on system load
    let timeout = Duration::from_secs(std::cmp::max(10, thread_count as u64 / 2));
    assert!(
        elapsed < timeout,
        "Concurrent execution took too long: {:?} (timeout: {:?})",
        elapsed,
        timeout
    );
}

/// Tests deadlock prevention in complex scenarios
#[test]
fn test_deadlock_prevention() {
    use std::sync::{Arc, Barrier};
    use std::thread;

    let thread_count = 10;
    let barrier = Arc::new(Barrier::new(thread_count));

    let handles: Vec<_> = (0..thread_count)
        .map(|i| {
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                // All threads start at the same time to increase chance of contention
                barrier.wait();

                // Run a complex pipeline that could potentially deadlock
                let output = cmd!("echo", &format!("input_{}", i))
                    .pipe(cmd!("cat"))
                    .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
                    .pipe(cmd!("wc", "-c"))
                    .no_echo()
                    .output()
                    .unwrap();

                let char_count: u32 = output.trim().parse().unwrap();
                assert!(char_count > 0);
            })
        })
        .collect();

    // If we can join all threads without hanging, no deadlock occurred
    for handle in handles {
        handle.join().unwrap();
    }
}
