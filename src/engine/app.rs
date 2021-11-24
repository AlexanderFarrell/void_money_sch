use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::ops::DerefMut;
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use crate::engine::engine::Engine;
use crate::WebGLEngine;

pub type Subversion = (u32, String);
pub struct Version {
    major: Subversion,
    minor: Subversion,
    revision: Subversion,
    build: Subversion,
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.major.0, self.minor.0, self.revision.0, self.build.0)
    }
}

pub trait CoreComponent {
    fn setup(&mut self);
    fn breakdown(&mut self);
}

pub trait Game {
    fn setup(&mut self, engine: &mut dyn Engine);
    fn update(&mut self, engine: &mut dyn Engine);
    fn breakdown(&mut self, engine: &mut dyn Engine);
    fn name(&self) -> String;
    fn version(&self) -> Version;
}

pub struct App<G: 'static + Game, E: 'static + Engine> {
    game: Rc<RefCell<G>>,
    engine: Rc<RefCell<E>>,
}

impl<G: 'static + Game, E: 'static + Engine> App<G, E> {
    pub fn new(game: G, engine: E) -> Self {
        App {
            game: Rc::new(RefCell::new(game)),
            engine: Rc::new(RefCell::new(engine))
        }
    }

    pub fn start(&mut self) {
        self.engine.borrow_mut().setup();
        {
            let mut i = self.engine.borrow_mut();
            self.game.borrow_mut().setup(i.deref_mut());
        }
        self.run();
    }

    pub fn end(&mut self) {
        //self.engine.stop();
        //self.game.borrow_mut().breakdown();
        //self.engine.borrow_mut().breakdown();
    }

    fn run(&self) {
        let mut game = self.game.clone();
        let mut e = self.engine.clone();
        let anim_frame = Rc::new(self.engine.borrow_mut().game_iter());
        let anim_frame_outer = anim_frame.clone();

        let inner_data_ref = Rc::new(RefCell::new(None));
        let outer_data_ref  = inner_data_ref.clone();

        *outer_data_ref.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let running: bool = e.clone().borrow_mut().running();

            if !running {
                let _ = inner_data_ref.borrow_mut().take();

                e.clone().borrow_mut().stop();
                {
                    let mut ee = e.borrow_mut();
                    game.clone().borrow_mut().breakdown(&mut *ee);
                }
                e.clone().borrow_mut().breakdown();

                return;
            }

            {
                let mut ee = e.borrow_mut();
                game.clone().borrow_mut().update(&mut *ee);
            }
            e.clone().borrow_mut().update();
            e.clone().borrow_mut().draw();

            anim_frame.clone()(inner_data_ref.borrow().as_ref().unwrap());

        }) as Box<dyn FnMut()>));

        anim_frame_outer.clone()(outer_data_ref.borrow().as_ref().unwrap());
    }
}