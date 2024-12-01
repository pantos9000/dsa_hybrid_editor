use std::sync::{mpsc, Arc};
use std::thread;

use crate::app::Drawable;
use crate::character::Character;

trait CharacterModification: FnMut(&mut Character) + Send + Sync + 'static {}

#[derive(Default, Clone)]
pub struct SimulatorBuilder {
    gradient_handles: Vec<GradientHandle>,
}

impl SimulatorBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_gradient(&mut self, modification: impl CharacterModification) -> Gradient {
        let modification = Arc::new(modification);
        let (handle, gradient) = create_gradient(modification);
        self.gradient_handles.push(handle);
        gradient
    }

    pub fn build(self) -> Simulator {
        Simulator::new(self.gradient_handles)
    }
}

#[derive(Debug)]
pub struct Simulator {
    thread: Option<thread::JoinHandle<()>>,
    command_send: mpsc::SyncSender<Command>,
}

impl Drop for Simulator {
    fn drop(&mut self) {
        let _ = self.command_send.send(Command::Stop);
        if let Some(thread) = self.thread.take() {
            thread.join().expect("thread panicked");
        }
    }
}

impl Simulator {
    fn new(gradient_handles: Vec<GradientHandle>) -> Self {
        let (command_send, command_recv) = mpsc::sync_channel(100);
        let thread = Self::spawn_thread(command_recv, gradient_handles);
        Self {
            thread: Some(thread),
            command_send,
        }
    }

    fn spawn_thread(
        command_recv: mpsc::Receiver<Command>,
        gradient_handles: Vec<GradientHandle>,
    ) -> thread::JoinHandle<()> {
        thread::Builder::new()
            .name("simulator_thread".to_owned())
            .spawn(move || {
                // TODO log
                'thread_loop: loop {
                    let command = command_recv.recv().expect("main thread is gone");
                    let (character, opponents) = match command {
                        Command::Simulate(data) => data,
                        Command::Stop => break 'thread_loop,
                    };
                    // first set all gradients to None to not show old data
                    for gradient in &gradient_handles {
                        gradient.invalidate();
                    }
                }
                todo!();
            })
            .expect("failed to spawn thread")
    }

    pub fn simulate(&mut self, character: Character, opponents: Vec<Character>) {
        self.command_send
            .send(Command::Simulate((character, opponents)))
            .expect("simulator thread is gone");
    }
}

enum Command {
    Simulate((Character, Vec<Character>)),
    Stop,
}

fn create_gradient(modification: Arc<dyn CharacterModification>) -> (GradientHandle, Gradient) {
    todo!()
}

#[derive(Debug)]
pub struct Gradient {
    recv: mpsc::Receiver<Option<GradientValue>>,
    value: Option<GradientValue>,
}

impl Gradient {
    pub fn update(&mut self) {
        let Ok(new_value) = self.recv.recv() else {
            return;
        };
        self.value = new_value;
    }
}

impl Drawable for Gradient {
    fn draw(&mut self, ui: &mut egui::Ui) {
        self.update();
        ui.label("gradient".to_string());
    }
}

/// Invariance: always between -100/100
#[derive(Debug, Default, Clone, Copy)]
struct GradientValue {
    left: i8,
    right: i8,
}

#[derive(Clone)]
struct GradientHandle {
    send: mpsc::SyncSender<Option<GradientValue>>,
    char_mod_func: Arc<dyn CharacterModification>,
}

impl GradientHandle {
    fn invalidate(&self) {
        self.send.send(None).expect("Gradient is gone");
    }

    fn update(&self, value: GradientValue) {
        self.send.send(Some(value)).expect("Gradient is gone");
    }
}
