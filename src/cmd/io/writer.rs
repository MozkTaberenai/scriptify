use crate::cmd::*;
use crate::*;

impl<W> Pipe<W> for Command
where
    W: std::io::Write + Sized,
{
    type In = Inherit;
    type Out = W;

    fn pipe(self, writer: W) -> Pipeline<Self::In, Self::Out> {
        Pipeline {
            quiet: self.quiet,
            inner: vec![self],
            input: Inherit,
            output: writer,
        }
    }
}

impl<I, W> Pipe<W> for Pipeline<I, Inherit>
where
    W: std::io::Write + Sized,
{
    type In = I;
    type Out = W;

    fn pipe(self, writer: W) -> Pipeline<Self::In, Self::Out> {
        Pipeline {
            inner: self.inner,
            input: self.input,
            output: writer,
            quiet: self.quiet,
        }
    }
}

impl<W: std::io::Write + Send + 'static> Spawn<ThreadHandle<W>> for Pipeline<Inherit, W> {
    fn spawn(mut self) -> Result<ThreadHandle<W>, Error> {
        let mut echo = echo(self.quiet);
        echo.put(&self);
        echo.put("|─▶".magenta());
        echo.end();

        let mut handle = self.inner_spawn(false, true)?;

        let mut reader = handle.take_stdout().unwrap();
        let mut writer = self.output;
        let thread_handle = std::thread::spawn(move || {
            std::io::copy(&mut reader, &mut writer)?;
            Ok(writer)
        });

        Ok(ThreadHandle {
            handle,
            thread: thread_handle,
        })
    }
}

impl<W: std::io::Write + Send + 'static> WriteSpawn<ThreadHandle<W>> for Pipeline<Inherit, W> {
    fn write_spawn(self) -> Result<(PipeStdin, ThreadHandle<W>), Error> {
        self.pipe_stdin().write_spawn()
    }
}

impl<W: std::io::Write + Send + 'static> Spawn<ThreadHandle<W>> for Pipeline<Piped, W> {
    fn spawn(mut self) -> Result<ThreadHandle<W>, Error> {
        let mut echo = echo(self.quiet);
        echo.put("─▶|".magenta());
        echo.put(&self);
        echo.put("|─▶".magenta());
        echo.end();

        let mut handle = self.inner_spawn(true, true)?;

        let mut reader = handle.take_stdout().unwrap();
        let mut writer = self.output;
        let thread_handle = std::thread::spawn(move || {
            std::io::copy(&mut reader, &mut writer)?;
            Ok::<_, std::io::Error>(writer)
        });

        Ok(ThreadHandle {
            handle,
            thread: thread_handle,
        })
    }
}

impl<W: std::io::Write + Send + 'static> WriteSpawn<ThreadHandle<W>> for Pipeline<Piped, W> {
    fn write_spawn(self) -> Result<(PipeStdin, ThreadHandle<W>), Error> {
        let mut handle = self.spawn()?;
        let stdin = handle.take_stdin().unwrap();
        Ok((stdin, handle))
    }
}
