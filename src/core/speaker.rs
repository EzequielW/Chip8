use rodio::{Sink, OutputStream, OutputStreamHandle};

pub(crate) struct Speaker{
    streamHandle: OutputStreamHandle,
    sink: Option<Sink>
}

impl Speaker{
    pub fn new() -> Speaker{
        let (_stream, streamHandle) = OutputStream::try_default().unwrap();
        Speaker {
            streamHandle,
            sink: None
        }
    }

    pub fn play(&self, frequency: Option<f32>){
        let freq = frequency.unwrap_or(440.0);
        let source = rodio::source::SineWave::new(freq);
        let sink = Sink::try_new(&self.streamHandle).unwrap();
        sink.append(source);
    }

    pub fn stop(&mut self){
        if !self.sink.is_none() {
            self.sink.as_ref().unwrap().stop();
            self.sink = None;
        }
    }
}