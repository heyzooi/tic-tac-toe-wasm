use yew::prelude::*;
use std::collections::HashMap;
include!("position.rs");
include!("player.rs");
extern crate web_sys;

#[derive(Properties, PartialEq)]
struct CellProps {
    position: Position,
    board: UseStateHandle<HashMap<Position, Player>>,
    class_name: Option<String>,
    current_player: UseStateHandle<Player>,
    winner: UseStateHandle<Option<Player>>,
}

#[function_component(Cell)]
fn cell(props: &CellProps) -> Html {
    let to_string = |player: Option<&Player>| {
        match player {
            Some(player) => player.to_string(),
            None => String::new(),
        }
    };
    
    let play = {
        let board = props.board.clone();
        let current_player = props.current_player.clone();
        let position = props.position.clone();
        let winner = props.winner.clone();
        Callback::from(move |_| {
            if winner.is_none() && !board.contains_key(&position) {
                let mut new_board: HashMap<Position, Player> = HashMap::with_capacity(board.len());
                new_board.clone_from(&board);
                let current_player_value = &*current_player;
                new_board.insert(position.clone(), current_player_value.clone());

                board.set(new_board.clone());

                let x1y1 = &new_board.get(&Position::X1Y1);
                let x2y1 = &new_board.get(&Position::X2Y1);
                let x3y1 = &new_board.get(&Position::X3Y1);

                let x1y2 = &new_board.get(&Position::X1Y2);
                let x2y2 = &new_board.get(&Position::X2Y2);
                let x3y2 = &new_board.get(&Position::X3Y2);

                let x1y3 = &new_board.get(&Position::X1Y3);
                let x2y3 = &new_board.get(&Position::X2Y3);
                let x3y3 = &new_board.get(&Position::X3Y3);

                let check = |cell_1: &Option<&Player>, cell_2: &Option<&Player>, cell_3: &Option<&Player>| {
                    if cell_1.is_some() && cell_2.is_some() && cell_3.is_some() {
                        match (cell_1, cell_2, cell_3) {
                            (Some(Player::X), Some(Player::X), Some(Player::X)) => Some(Player::X),
                            (Some(Player::O), Some(Player::O), Some(Player::O)) => Some(Player::O),
                            _ => None
                        } 
                    } else {
                        None
                    }
                };

                if let Some(player) = check(x1y1, x2y1, x3y1) {
                    winner.set(Some(player));
                } else if let Some(player) = check(x1y2, x2y2, x3y2) {
                    winner.set(Some(player));
                } else if let Some(player) = check(x1y3, x2y3, x3y3) {
                    winner.set(Some(player));
                } else if let Some(player) = check(x1y1, x1y2, x1y3) {
                    winner.set(Some(player));
                } else if let Some(player) = check(x2y1, x2y2, x2y3) {
                    winner.set(Some(player));
                } else if let Some(player) = check(x3y1, x3y2, x3y3) {
                    winner.set(Some(player));
                } else if let Some(player) = check(x1y1, x2y2, x3y3) {
                    winner.set(Some(player));
                } else if let Some(player) = check(x3y1, x2y2, x1y3) {
                    winner.set(Some(player));
                } else {
                    current_player.set(if *current_player == Player::X {
                        Player::O
                    } else {
                        Player::X
                    });
                }
            }
        })
    };

    let additional_classes = match &props.class_name {
        Some(class_name) => class_name.clone(),
        None => String::new(),
    };

    html! {
        <div class={format!("cell {additional_classes}")} onclick={play}>{to_string(props.board.get(&props.position))}</div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let board = use_state_eq(|| HashMap::new());
    let current_player = use_state_eq(|| Player::X);
    let winner: UseStateHandle<Option<Player>> = use_state_eq(|| None);

    let reset = {
        let board = board.clone();
        let current_player = current_player.clone();
        let winner = winner.clone();
        Callback::from(move |_| {
            board.set(HashMap::new());
            current_player.set(Player::X);
            winner.set(None);
        })
    };

    html! {
        <div class="container">
            {match &*winner { 
                Some(player) => html! {
                    <div>{format!("Winner: {}", player.to_string())}</div>
                },
                None => html! {
                    <div>{format!("Current Player: {}", current_player.to_string())}</div>
                },
            }}
            <div class="inner-container">
                <Cell position={Position::X1Y1} board={board.clone()} current_player={current_player.clone()} winner={winner.clone()} />
                <Cell position={Position::X2Y1} board={board.clone()} current_player={current_player.clone()} winner={winner.clone()} class_name="cell-x"/>
                <Cell position={Position::X3Y1} board={board.clone()} current_player={current_player.clone()} winner={winner.clone()} />
                
                <Cell position={Position::X1Y2} board={board.clone()} current_player={current_player.clone()} winner={winner.clone()} class_name="cell-y"/>
                <Cell position={Position::X2Y2} board={board.clone()} current_player={current_player.clone()} winner={winner.clone()} class_name="cell-center"/>
                <Cell position={Position::X3Y2} board={board.clone()} current_player={current_player.clone()} winner={winner.clone()} class_name="cell-y"/>

                <Cell position={Position::X1Y3} board={board.clone()} current_player={current_player.clone()} winner={winner.clone()} />
                <Cell position={Position::X2Y3} board={board.clone()} current_player={current_player.clone()} winner={winner.clone()} class_name="cell-x"/>
                <Cell position={Position::X3Y3} board={board.clone()} current_player={current_player.clone()} winner={winner.clone()} />
            </div>
            {match &*winner { 
                Some(_) => html! {
                    <button onclick={reset}>{"Play again"}</button>
                },
                None => html! {
                    <button onclick={reset}>{"Reset Game"}</button>
                },
            }}
        </div>
    }
}
