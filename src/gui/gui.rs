use iced::{
    Alignment, Length,
    widget::{Column, Row, button, column, container, row, text_input},
};

use crate::sudoku::grid::{Cell, CellState, Coordinate, Grid};

#[derive(Default)]
pub struct Program {
    grid: Grid,
    mode: Mode,
}

#[derive(Default)]
enum Mode {
    #[default]
    None,
    Playing,
    Solving,
}

#[derive(Debug, Clone)]
enum Message {
    ClearGrid,
    RandomiseGrid,
    SolveGrid,
    ChangedNumber(Coordinate, String),
}

impl Program {
    pub fn run() -> iced::Result {
        iced::application("Sudoku", Program::update, Program::view)
            .theme(|_| Program::theme())
            .run()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ClearGrid => self.grid = Grid::new_empty(),
            Message::RandomiseGrid => self.grid = Grid::new_random(),
            Message::SolveGrid => self.grid.solve(),
            Message::ChangedNumber(coordinate, s) => {
                let (new_state, modifiable) = if s.is_empty() {
                    (CellState::Empty, true)
                } else if let Ok(value) = s.parse() {
                    (CellState::Filled(value), false)
                } else {
                    return;
                };

                if let Ok(_) = self.grid.set_cell(coordinate, new_state) {
                    let _ = self.grid.set_cell_solver_modifiable(coordinate, modifiable);
                }
            }
        }
    }

    fn view(&self) -> iced::Element<Message> {
        container(
            column![self.grid_view(), self.controls()]
                .align_x(Alignment::Center)
                .spacing(10),
        )
        .align_y(Alignment::Center)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn controls(&self) -> iced::Element<Message> {
        row![
            button("Clear").on_press(Message::ClearGrid),
            button("Randomise").on_press(Message::RandomiseGrid),
            button("Solve").on_press(Message::SolveGrid)
        ]
        .spacing(10)
        .into()
    }

    fn grid_view(&self) -> iced::Element<Message> {
        Column::with_children(
            self.grid
                .cells
                .chunks(Grid::COL_COUNT)
                .map(|cells| self.grid_row(cells)),
        )
        .spacing(10)
        .into()
    }

    fn grid_row<'a>(&self, cells: &'a [Cell]) -> iced::Element<'a, Message> {
        Row::with_children(cells.iter().map(|c| self.cell_view(c)))
            .spacing(10)
            .into()
    }

    fn cell_view<'a>(&self, c: &'a Cell) -> iced::Element<'a, Message> {
        text_input(
            &format!(
                "{}",
                match c.state {
                    CellState::Filled(value) => value.to_string(),
                    _ => String::new(),
                }
            ),
            "",
        )
        .on_input(|s| Message::ChangedNumber(c.coordinate, s))
        .width(30)
        .align_x(Alignment::Center)
        .into()
    }

    fn theme() -> iced::Theme {
        iced::Theme::Dark
    }
}
