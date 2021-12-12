mod map;
mod map_builder;
mod camera;
mod components;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub const SCREEN_WIDTH:i32 = 80;
    pub const SCREEN_HEIGHT:i32 = 50;
    pub const DISPLAY_WIDHT:i32 = SCREEN_WIDTH/2;
    pub const DISPLAY_HEIGHT:i32 = SCREEN_HEIGHT/2;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use crate::prelude::*;

struct State {
//    map: Map,
//    player: Player,
//    camera: Camera,
      ecs: World,
      resources: Resources,
      input_systems: Schedule,
      player_systems: Schedule,
      monster_systems: Schedule
}
impl State {
//    fn new()-> State{
//        let mut rng = RandomNumberGenerator::new();
//        let mb = MapBuilder::new(&mut rng);
//        State{
//            map:mb.map,
//            player: Player::new(mb.player_start),
//            camera: Camera::new(mb.player_start),
//        }
//    }
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mb = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, mb.player_start);
        mb.rooms.iter().skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));
        resources.insert(mb.map);
        resources.insert(Camera::new(mb.player_start));
       resources.insert(TurnState::AwaitingInput); 
            Self {
                ecs,
                resources,
                input_systems: build_input_scheduler(),
                player_systems: build_player_scheduler(),
                monster_systems: build_monster_scheduler()
            }
    }
}

impl GameState for State {
   fn tick(&mut self, ctx:&mut BTerm){
       ctx.set_active_console(0);
       ctx.cls();
       ctx.set_active_console(1);
       ctx.cls();
       //self.player.update(ctx, &self.map,&mut self.camera);
       //self.map.render(ctx,&self.camera);
       //self.player.render(ctx,&self.camera);
       //TODO: Execute Systems
       self.resources.insert(ctx.key);

       let current_state = self.resources.get::<TurnState>().unwrap().clone();
       match current_state {
           TurnState::AwaitingInput => self.input_systems.execute(&mut self.ecs, &mut self.resources),
           TurnState::PlayerTurn => self.player_systems.execute(&mut self.ecs,&mut self.resources ),
           TurnState::MonsterTurn => self.monster_systems.execute(&mut self.ecs,&mut self.resources)
       }
       //TODO: Render Draw Buffer
       render_draw_buffer(ctx).expect("Render Error");
   }
}

fn main() -> BError {
//let context = BTermBuilder::simple80x50().with_title("Dungeon Crawler").with_fps_cap(30.0).build()?;
let context = BTermBuilder::new()
             .with_title("Dungeon Crawler")
             .with_fps_cap(30.0)
             .with_dimensions(DISPLAY_WIDHT, DISPLAY_HEIGHT)
             .with_tile_dimensions(32, 32)
             .with_resource_path("resources/")
             .with_font("dungeonfont.png", 32, 32)
             .with_simple_console(DISPLAY_WIDHT, DISPLAY_HEIGHT, "dungeonfont.png")
             .with_simple_console_no_bg(DISPLAY_WIDHT, DISPLAY_HEIGHT, "dungeonfont.png")
             .build()?;

                
main_loop(context, State::new())
}
