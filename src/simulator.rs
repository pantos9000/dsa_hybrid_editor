use std::sync::atomic::{AtomicI8, AtomicU8, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;

use crate::character::Character;

pub type CharacterModification = Box<dyn FnMut(&mut Character) + Send>;

pub struct SimulatorBuilder {
    gradient_data: Vec<GradientData>,
}

impl SimulatorBuilder {
    #[allow(dead_code)] // TODO
    pub fn add_gradient(&mut self, modification: CharacterModification) -> Gradient {
        let (gradient, handle) = create_gradient();
        let gradient_data = GradientData {
            handle,
            modification,
        };
        self.gradient_data.push(gradient_data);
        gradient
    }

    #[allow(dead_code)] // TODO
    pub fn build(self) -> Simulator {
        Simulator::new(self.gradient_data)
    }
}

pub struct Simulator {
    thread: Option<thread::JoinHandle<()>>,
    send: Option<mpsc::Sender<CharData>>,
    progress: Arc<AtomicU8>,
}

impl Drop for Simulator {
    fn drop(&mut self) {
        drop(self.send.take());
        if let Some(thread) = self.thread.take() {
            thread.join().expect("worker thread panicked");
        }
    }
}

impl Simulator {
    fn new(gradient_data: Vec<GradientData>) -> Self {
        let (send, recv) = mpsc::channel();
        let progress = Arc::new(AtomicU8::new(0));
        let thread = Some(Self::spawn_simulator_thread(gradient_data, recv, &progress));
        Self {
            thread,
            send: Some(send),
            progress,
        }
    }

    fn spawn_simulator_thread(
        gradient_data: Vec<GradientData>,
        recv: mpsc::Receiver<CharData>,
        progress: &Arc<AtomicU8>,
    ) -> thread::JoinHandle<()> {
        let progress = Arc::clone(progress);
        thread::Builder::new()
            .name("simulator_thread".to_owned())
            .spawn(|| {
                log::info!("starting simulator thread");
                Self::simulator_thread(gradient_data, recv, progress);
                log::info!("stopping simulator thread");
            })
            .expect("failed to spawn thread")
    }

    fn simulator_thread(
        gradient_data: Vec<GradientData>,
        recv: mpsc::Receiver<CharData>,
        progress: Arc<AtomicU8>,
    ) {
        let mut recv = recv.iter().peekable();
        'thread_loop: loop {
            let Some(char_data) = recv.next() else {
                log::debug!("channel is closed, exiting thread loop");
                return;
            };

            // we got a new configuration, so void all current results
            for GradientData {
                handle,
                modification: _,
            } in &gradient_data
            {
                handle.store_value(None);
            }

            // now do the calculations
            for (
                i,
                GradientData {
                    handle,
                    modification,
                },
            ) in gradient_data.iter().enumerate()
            {
                // do calculation and store it
                let gradient = Self::dummy_calculation(&char_data, modification);
                handle.store_value(Some(gradient));

                // update progress
                let current_progress = 100 * i / gradient_data.len();
                let current_progress = current_progress
                    .try_into()
                    .expect("progress percentage did not fit into a u8");
                progress.store(current_progress, Ordering::Relaxed);

                // cancel inner loop if new char configuration is available
                if recv.peek().is_some() {
                    continue 'thread_loop;
                }
            }
        }
    }

    fn dummy_calculation(_char_data: &CharData, _modification: &CharacterModification) -> i8 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        42
    }

    #[allow(dead_code)] // TODO
    pub fn update_characters(&self, character: Character, enemy: Character) {
        let char_data = CharData { character, enemy };
        self.send
            .as_ref()
            .expect("sender does not exist")
            .send(char_data)
            .expect("worker thread is gone");
    }

    /// Return the progress of current calculations
    #[allow(dead_code)] // TODO
    pub fn progress(&self) -> u8 {
        self.progress.load(Ordering::Relaxed)
    }
}

fn create_gradient() -> (Gradient, GradientHandle) {
    let value = Arc::new(AtomicI8::new(i8::MIN));
    let gradient_handle = GradientHandle {
        value: Arc::clone(&value),
    };
    let gradient = Gradient { value };
    (gradient, gradient_handle)
}

pub struct Gradient {
    value: Arc<AtomicI8>,
}

impl Gradient {
    #[allow(dead_code)] // TODO
    pub fn value(&self) -> Option<i8> {
        let loaded = self.value.load(Ordering::Relaxed);
        match loaded {
            ..-100 => unreachable!(),
            101..i8::MAX => unreachable!(),
            i8::MAX => None,
            x => Some(x),
        }
    }
}

struct GradientHandle {
    value: Arc<AtomicI8>,
}

impl GradientHandle {
    fn store_value(&self, value: Option<i8>) {
        let store = match value {
            None => i8::MAX,
            Some(..-100) => unreachable!(),
            Some(101..) => unreachable!(),
            Some(x) => x,
        };
        self.value.store(store, Ordering::Relaxed);
    }
}

struct CharData {
    character: Character,
    enemy: Character,
}

struct GradientData {
    handle: GradientHandle,
    modification: CharacterModification,
}

// TODO tests
