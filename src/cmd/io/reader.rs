use crate::cmd::*;
use crate::*;

impl<R> Pipe<Command> for R
where
    R: std::io::Read + Sized,
{
    type In = Self;
    type Out = Inherit;

    fn pipe(self, to: Command) -> Pipeline<Self::In, Self::Out> {
        Pipeline {
            inner: vec![to],
            input: self,
            output: Inherit,
            quiet: false,
        }
    }
}

impl<R: std::io::Read + Send + 'static> Spawn<ThreadHandle<()>> for Pipeline<R, Inherit> {
    fn spawn(mut self) -> Result<ThreadHandle<()>, Error> {
        let mut echo = echo(self.quiet);
        echo.put("─▶|".magenta());
        echo.put(&self);
        echo.end();

        let mut handle = self.inner_spawn(true, false)?;

        let mut reader = self.input;
        let mut writer = handle.take_stdin().unwrap();
        let thread_handle = std::thread::spawn(move || {
            std::io::copy(&mut reader, &mut writer)?;
            Ok(())
        });

        Ok(ThreadHandle {
            handle,
            thread: thread_handle,
        })
    }
}

impl<R: std::io::Read + Send + 'static> Pipeline<R, Inherit> {
    pub fn run(self) -> Result<Status, Error> {
        self.spawn()?.wait().map(|x| x.0)
    }
}

impl<R: std::io::Read + Send + 'static> ReadSpawn<ThreadHandle<()>> for Pipeline<R, Inherit> {
    fn read_spawn(self) -> Result<(PipeStdout, ThreadHandle<()>), Error> {
        self.pipe_stdout().read_spawn()
    }
}

impl<R: std::io::Read + Send + 'static> Spawn<ThreadHandle<()>> for Pipeline<R, Piped> {
    fn spawn(mut self) -> Result<ThreadHandle<()>, Error> {
        let mut echo = echo(self.quiet);
        echo.put("─▶|".magenta());
        echo.put(&self);
        echo.put("|─▶".magenta());
        echo.end();

        let mut handle = self.inner_spawn(true, true)?;

        let mut reader = self.input;
        let mut writer = handle.take_stdin().unwrap();
        let thread_handle = std::thread::spawn(move || {
            std::io::copy(&mut reader, &mut writer)?;
            Ok(())
        });

        Ok(ThreadHandle {
            handle,
            thread: thread_handle,
        })
    }
}

impl<R: std::io::Read + Send + 'static> ReadSpawn<ThreadHandle<()>> for Pipeline<R, Piped> {
    fn read_spawn(self) -> Result<(PipeStdout, ThreadHandle<()>), Error> {
        let mut handle = self.spawn()?;
        let stdout = handle.take_stdout().unwrap();
        Ok((stdout, handle))
    }
}

impl<T> ReadSpawnExt<ThreadHandle<()>> for T
where
    T: ReadSpawn<ThreadHandle<()>>,
{
    fn read_to_end(self) -> Result<Vec<u8>, Error> {
        let (mut stdout, handle) = self.read_spawn()?;
        let vec = stdout.read_to_end()?;
        handle.wait()?;
        Ok(vec)
    }

    fn read_to_string(self) -> Result<String, Error> {
        let (mut stdout, handle) = self.read_spawn()?;
        let string = stdout.read_to_string()?;
        handle.wait()?;
        Ok(string)
    }
}
