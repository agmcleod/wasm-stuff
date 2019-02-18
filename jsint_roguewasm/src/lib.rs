#[macro_use]
extern crate serde_derive;
use wasm_bindgen::prelude::*;
use js_sys;

use std::collections::HashMap;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(module = "./index")]
    fn stats_updated(stats: JsValue);

    pub type Display;

    #[wasm_bindgen(method, structural, js_namespace = ROT)]
    fn draw(this: &Display, x: i32, y: i32, ch: &str);

    #[wasm_bindgen(method, structural, js_name = draw, js_namespace = ROT)]
    fn draw_color(this: &Display, x: i32, y: i32, ch: &str, color: &str);

    pub type RNG;

    #[wasm_bindgen(method, structural, js_name = getUniform, js_namespace = ROT)]
    fn get_uniform(this: &RNG) -> usize;
}

#[wasm_bindgen]
pub struct Engine {
    display: Display,
    rng: RNG,
    points: HashMap<GridPoint, String>,
    prize_location: Option<GridPoint>,
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new(rng: RNG, display: Display) -> Engine {
        Engine{
            display,
            rng,
            points: HashMap::new(),
            prize_location: None,
        }
    }

    pub fn generate_boxes(&mut self, free_cells: js_sys::Array) -> JsValue {
        let mut free_cells: Vec<[i32; 2]> = free_cells.into_serde().unwrap();
        for i in 0..10 {
            let index = self.rng.get_uniform() * free_cells.len();
            let coords = free_cells.remove(index);
            let mut parts = coords.iter();
            let x: i32 = *parts.next().unwrap();
            let y: i32 = *parts.next().unwrap();

            self.place_box(x, y);
            if i == 9 {
                self.mark_wasmprize(x, y);
            }
        }

        let as_strings: Vec<String> = free_cells.iter().map(|coords| {
            format!("{},{}", coords[0], coords[1])
        }).collect();

        JsValue::from_serde(&as_strings).unwrap()
    }

    pub fn on_dig(&mut self, x: i32, y: i32, val: i32) {
        if val == 0 {
            let pt = GridPoint{ x, y };
            self.points.insert(pt, ".".to_owned());
        }
    }

    pub fn draw_map(&self) {
        for (k, v) in &self.points {
            self.display.draw(k.x, k.y, &v);
        }
    }

    pub fn redraw_at(&self, x: i32, y: i32) {
        let g = GridPoint { x, y };
        if let Some(v) = self.points.get(&g) {
            self.display.draw(x, y, v);
        }
    }

    pub fn place_box(&mut self, x: i32, y: i32) {
        let g = GridPoint{ x, y };
        self.points.insert(g, "*".to_owned());
    }

    pub fn open_box(&mut self, pc: &mut PlayerCore, x: i32, y: i32) {
        let spot = GridPoint { x, y };
        {
            let v = self.points.get(&spot).unwrap();

            if v != "*" {
                alert("There's no prize box here.");
                return;
            }
        }

        if let Some(ref loc) = self.prize_location {
            if *loc == spot {
                // YAY! you win!
                alert("Congratulations! You've found the WebAssembly Module!");
            } else {
                alert("Woops! This was a booby trap!");
                pc.take_damage(30);
            }
        }

        self.remove_box(spot.x, spot.y);
    }

    fn remove_box(&mut self, x: i32, y: i32) {
        let loc = GridPoint { x, y };
        self.points.insert(loc, ".".to_owned());
    }

    pub fn mark_wasmprize(&mut self, x: i32, y: i32) {
        let g = GridPoint { x, y };
        if let Some(v) = self.points.get(&g) {
            if v == "*" {
                self.prize_location = Some(g);
            }
        }
    }

    pub fn move_player(&mut self, pc: &mut PlayerCore, x: i32, y: i32) {
        // replace player icon with what's underneath
        self.redraw_at(pc.x(), pc.y());

        pc.move_to(x, y);
    }

    pub fn free_cell(&self, x: i32, y: i32) -> bool {
        let g = GridPoint{ x, y };
        match self.points.get(&g) {
            Some(v) => v == "." || v == "*",
            None => false,
        }
    }
}

#[derive(Serialize)]
pub struct Stats {
    pub hitpoints: i32,
    pub max_hitpoints: i32,
    pub moves: i32,
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug, Hash)]
struct GridPoint {
    pub x: i32,
    pub y: i32,
}

#[wasm_bindgen]
pub struct PlayerCore {
    loc: GridPoint,
    moves: i32,
    display: Display,
    hp: i32,
    max_hp: i32,
    icon: String,
    color: String,
}

#[wasm_bindgen]
impl PlayerCore {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i32, y: i32, icon: &str, color: &str, display: Display) -> Self {
        PlayerCore{
            loc: GridPoint{ x, y},
            display,
            moves: 0,
            max_hp: 100,
            hp: 100,
            icon: icon.to_owned(),
            color: color.to_owned(),
        }
    }

    pub fn x(&self) -> i32 {
        self.loc.x
    }

    pub fn y(&self) -> i32 {
        self.loc.y
    }

    pub fn draw(&self) {
        self.display
            .draw_color(self.loc.x, self.loc.y, &self.icon, &self.color);
    }

    pub fn move_to(&mut self, x: i32, y: i32) {
        self.loc = GridPoint{ x, y };
        self.draw();
        self.moves += 1;
        self.emit_stats();
    }

    pub fn emit_stats(&self) {
        let stats = Stats {
            hitpoints: self.hp,
            max_hitpoints: self.max_hp,
            moves: self.moves,
        };

        stats_updated(JsValue::from_serde(&stats).unwrap());
    }

    pub fn take_damage(&mut self, hits: i32) -> i32 {
        self.hp -= hits;
        self.emit_stats();
        self.hp
    }
}
