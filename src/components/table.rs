pub enum Size {
    Cells(usize),
}

pub struct Column {
    pub component: Box<dyn crate::core::render::Render>,
    pub size:      Size,
}

pub struct Table {
    columns: Vec<Column>,
    // rows:    Vec<Box<dyn crate::core::render::Render>>,
}

impl crate::core::render::Render for Table {
    fn render(
        &self,
        options: &crate::core::render::Options,
    ) -> crate::core::render::Segments {
        let mut segments = Vec::new();

        let column_sizes: Vec<usize> = self
            .columns
            .iter()
            .map(|column| match column.size {
                Size::Cells(cells) => cells,
            })
            .collect();
        let mut columns_segments: Vec<crate::core::render::Segments> =
            Vec::with_capacity(self.columns.len());

        for column in self.columns.iter() {
            columns_segments.push(column.component.render(
                &crate::core::render::Options {
                    columns: match column.size {
                        Size::Cells(columns) => columns,
                    },
                    ..*options
                },
            ));
        }
        for x in columns_segments.iter() {
            println!("{:?}", x);
        }

        let mut index: usize = 0;
        let mut done: usize = 0;
        while done < columns_segments.len() {
            for (columns_index, column_segments) in
                columns_segments.iter().enumerate()
            {
                if column_segments.len() == index {
                    done += 1;
                }
                else if index < column_segments.len() {
                    segments.push(crate::core::render::Segment {
                        style: column_segments[index].style,
                        text:  column_segments[index].text.clone(),
                    });
                }
                else {
                    segments.push(crate::core::render::Segment {
                        style: column_segments[column_segments.len() - 1].style,
                        text:  std::iter::repeat(" ")
                            .take(column_sizes[columns_index])
                            .collect::<String>(),
                    });
                }
            }
            index += 1;
            segments.push(crate::core::render::Segment {
                style: crate::core::style::Style::new(),
                text:  String::from("\n"),
            });
        }

        segments
    }
}

impl Table {
    pub fn new() -> Table {
        Table { columns: vec![] /* ,rows: vec![] */ }
    }

    pub fn add_column(
        &mut self,
        column: Column,
    ) {
        self.columns.push(column);
    }

    // pub fn add_row(
    //     &mut self,
    //     rows: Vec<Box<dyn crate::core::render::Render>>,
    // ) {
    //     self.rows.push(column);
    // }
}

#[cfg(test)]
mod test_table {
    use super::{Column, Size, Table};

    #[test]
    fn new() {}
}
