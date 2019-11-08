/*
    Copyright 2010-2011 Ilkka Halila
    Copyright 2019 Alexander Krivács Schrøder

    This file is part of Goblin Camp Revival.

    Goblin Camp Revival is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Goblin Camp Revival is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Goblin Camp Revival.  If not, see <https://www.gnu.org/licenses/>.
*/

pub mod loading_dialog;

use crate::data::base::{Position, Size};
use crate::game::game_data::{MapGenerationState, MapRenderData};
use crate::game::game_state::game::loading_dialog::LoadingDialog;
use crate::game::game_state::{
    GameState, GameStateBackgroundUpdateResult, GameStateChange, GameStateError, GameStateResult,
    GameStateUpdateResult,
};
use crate::game::GameRef;
use crate::ui::MessageBox;
use slog::o;
use std::borrow::Cow;
use tcod::{BackgroundFlag, Console};

pub struct ConfirmNewGame;

impl ConfirmNewGame {
    pub fn game_state_change(_: &mut GameRef) -> GameStateChange {
        GameStateChange::Push(Box::new(ConfirmNewGame))
    }
}

impl GameState for ConfirmNewGame {
    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed("Confirm new game")
    }

    fn update(&mut self, game_ref: &mut GameRef) -> GameStateUpdateResult {
        let logger = game_ref.logger.clone();
        if game_ref.game_data.running {
            Ok(GameStateChange::PopPush(MessageBox::game_state(
                game_ref,
                "A game is already running, are you sure you want  to start a new one?",
                "Yes",
                Box::new(|| GameStateChange::EndGame),
                Some("No"),
                Some(Box::new(|| Game::game_state_change(logger))),
            )))
        } else {
            Ok(Game::game_state_change(logger))
        }
    }

    fn draw(&mut self, _: &mut GameRef) -> GameStateResult {
        Ok(())
    }
}

pub struct Game {
    logger: slog::Logger,
    first_run: bool,
    map_generation_state: Option<MapGenerationState>,
    camera: Position,
}

impl Game {
    pub fn game_state_change(parent_logger: slog::Logger) -> GameStateChange {
        GameStateChange::Push(Box::new(Game {
            logger: parent_logger.new(o!()),
            first_run: true,
            map_generation_state: None,
            camera: Position::ORIGIN + (110, 75),
        }))
    }
}

impl GameState for Game {
    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed("Game")
    }

    fn background_update(&mut self, game_ref: &mut GameRef) -> GameStateBackgroundUpdateResult {
        if self.first_run {
            if self.map_generation_state.is_none() {
                game_ref.data.generator.reseed_with_default();
                game_ref.game_data.reset();
                self.map_generation_state = game_ref.game_data.generate_map(
                    &mut game_ref.data.generator,
                    &game_ref.data.settings,
                    None,
                );
                return Ok(None);
            }
            let state = self.map_generation_state.as_mut().unwrap();
            if !state.is_done() {
                game_ref.game_data.generate_map(
                    &mut game_ref.data.generator,
                    &game_ref.data.settings,
                    Some(state),
                );

                Ok(None)
            } else {
                self.first_run = false;
                self.map_generation_state.take();

                Ok(Some("DoneLoading".to_string()))
            }
        /*
            game->SetSeason(EarlySpring);

            std::priority_queue<std::pair<int, Coordinate> > spawnCenterCandidates;

            for (int tries = 0; tries < 20; ++tries) {
                std::pair<int,Coordinate> candidate(0, Random::ChooseInExtent(zero+100, Map::Inst()->Extent() - 100));

                int riverDistance = 1000, hillDistance = 1000;
                for (int i = 0; i < 4; ++i) {
                    Direction dirs[4] = { WEST, EAST, NORTH, SOUTH };
                    int distance = 200;
                    Coordinate p = candidate.second + Coordinate::DirectionToCoordinate(dirs[i]) * distance;
                    TCODLine::init(p.X(), p.Y(), candidate.second.X(), candidate.second.Y());
                    do {
                        if (Map::Inst()->IsInside(p)) {
                            if (Map::Inst()->GetType(p) == TILEDITCH || Map::Inst()->GetType(p) == TILERIVERBED) {
                                if (distance < riverDistance) riverDistance = distance;
                                if (distance < 25) riverDistance = 2000;
                            } else if (Map::Inst()->GetType(p) == TILEROCK) {
                                if (distance < hillDistance) hillDistance = distance;
                            }
                        }
                        --distance;
                    } while (!TCODLine::step(p.Xptr(), p.Yptr()));
                }

                candidate.first = -hillDistance - riverDistance;
                if (Map::Inst()->GetType(candidate.second) != TILEGRASS) candidate.first -= 10000;
                spawnCenterCandidates.push(candidate);
            }

            Coordinate spawnTopCorner = spawnCenterCandidates.top().second - 20;
            Coordinate spawnBottomCorner = spawnCenterCandidates.top().second + 20;

            //Clear starting area
            for (int x = spawnTopCorner.X(); x < spawnBottomCorner.X(); ++x) {
                for (int y = spawnTopCorner.Y(); y < spawnBottomCorner.Y(); ++y) {
                    Coordinate p(x,y);
                    if (Map::Inst()->GetNatureObject(p) >= 0 && Random::Generate(2) < 2) {
                        game->RemoveNatureObject(game->natureList[Map::Inst()->GetNatureObject(p)]);
                    }
                }
            }

            //we use Top+15, Bottom-15 to restrict the spawning zone of goblin&orc to the very center, instead of spilled over the whole camp
            game->CreateNPCs(15, NPC::StringToNPCType("goblin"), spawnTopCorner+15, spawnBottomCorner-15);
            game->CreateNPCs(6, NPC::StringToNPCType("orc"), spawnTopCorner+15, spawnBottomCorner-15);

            game->CreateItems(30, Item::StringToItemType("Bloodberry seed"), spawnTopCorner, spawnBottomCorner);
            game->CreateItems(5, Item::StringToItemType("Blueleaf seed"), spawnTopCorner, spawnBottomCorner);
            game->CreateItems(30, Item::StringToItemType("Nightbloom seed"), spawnTopCorner, spawnBottomCorner);
            game->CreateItems(20, Item::StringToItemType("Bread"), spawnTopCorner, spawnBottomCorner);

            //we place two corpses on the map
            Coordinate corpseLoc[2];

            //find suitable location
            for (int c = 0; c < 2; ++c) {
                Coordinate p;
                do {
                    p = Random::ChooseInRectangle(spawnTopCorner, spawnBottomCorner);
                } while(!Map::Inst()->IsWalkable(p));
                corpseLoc[c] = p;
            }

            //initialize corpses
            for (int c = 0; c < 2; ++c) {
                game->CreateItem(corpseLoc[c], Item::StringToItemType("stone axe"));
                game->CreateItem(corpseLoc[c], Item::StringToItemType("shovel"));
                int corpseuid = game->CreateItem(corpseLoc[c], Item::StringToItemType("corpse"));
                boost::shared_ptr<Item> corpse = game->itemList[corpseuid];
                corpse->Name("Corpse(Human woodsman)");
                corpse->Color(TCODColor::white);
                for (int i = 0; i < 6; ++i)
                    game->CreateBlood(Random::ChooseInRadius(corpseLoc[c], 2));
            }

            Camp::Inst()->SetCenter(spawnCenterCandidates.top().second);
            game->CenterOn(spawnCenterCandidates.top().second);

            Map::Inst()->SetTerritoryRectangle(spawnTopCorner, spawnBottomCorner, true);

            Map::Inst()->weather->ApplySeasonalEffects();

            for (int i = 0; i < 10; ++i)
                Game::Inst()->events->SpawnBenignFauna();
        */
        } else {
            Ok(None)
        }
    }

    fn update(&mut self, game_ref: &mut GameRef) -> GameStateUpdateResult {
        if self.first_run {
            Ok(LoadingDialog::game_state_change())
        } else {
            /*
                Game* game = Game::Inst();
                if (!game->Running()) {
                    Announce::Inst()->AddMsg("Press 'h' for keyboard shortcuts", TCODColor::cyan);
                }
                game->Running(true);

                int update = -1;
                if (Config::GetCVar<int>("halfRendering")) update = 0;

                int elapsedMilli;
                int targetMilli = 1000 / (UPDATES_PER_SECOND);
                int startMilli = TCODSystem::getElapsedMilli();
                while (game->Running()) {
                    if (Game::ToMainMenu()) {
                        Game::ToMainMenu(false);
                        return;
                    }

                    UI::Inst()->Update();
                    if (!game->Paused()) {
                        game->Update();
                        Announce::Inst()->Update();
                    }

                    if (update <= 0) {
                        game->Draw();
                        game->FlipBuffer();
                        if (update == 0) update = 1;
                    } else if (update == 1) update = 0;

                    elapsedMilli = TCODSystem::getElapsedMilli() - startMilli;
                    startMilli = TCODSystem::getElapsedMilli();
                    if (elapsedMilli < targetMilli) TCODSystem::sleepMilli(targetMilli - elapsedMilli);
                }

                Script::Event::GameEnd();
            */

            Ok(GameStateChange::None)
        }
    }

    fn draw(&mut self, game_ref: &mut GameRef) -> Result<(), GameStateError> {
        game_ref.root.set_background_flag(BackgroundFlag::Set);

        let size_x = game_ref.root.width();
        let size_y = game_ref.root.height();
        let (char_x, char_y) = tcod::system::get_char_size();

        let render_data = MapRenderData::new(
            self.camera,
            Position::new(0, 0) + Size::new(size_x * char_x, size_y * char_y),
            game_ref.root,
        );

        game_ref.game_data.render_map(render_data);

        /*
            TCODConsole * console = Game::Inst()->buffer,
            float focusX = Game::Inst()->camX, float focusY = Game::Inst()->camY,
            bool drawUI = true, int posX = 0, int posY = 0, int xSize = -1, int ySize = -1
            console->setBackgroundFlag(TCOD_BKGND_SET);
            renderer->DrawMap(Map::Inst(), focusX, focusY, posX * charX, posY * charY, sizeX * charX, sizeY * charY);

            if (drawUI) {
                UI::Inst()->Draw(console);
            }
        */

        Ok(())
    }
}
