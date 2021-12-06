pub enum Size {
    Cells(usize),
    Weight(f64),
}

pub struct Table {
    pub column_sizes: Vec<Size>,
    pub rows:         Vec<Vec<Box<dyn crate::core::render::Render>>>,
}

impl crate::core::render::Render for Table {
    fn render(
        &self,
        options: &crate::core::render::Options,
    ) -> crate::core::segment::RenderedSegments {
        let columns = match options.columns {
            Some(columns) => columns,
            None => crate::core::render::DEFAULT_COLUMNS,
        };

        let mut rendered_segments = Vec::new();

        let total_weight: f64 = self
            .column_sizes
            .iter()
            .map(|column_size| match column_size {
                Size::Weight(weight) => *weight,
                _ => 0.0,
            })
            .sum();
        let total_cells: usize = self
            .column_sizes
            .iter()
            .map(|column_size| match column_size {
                Size::Cells(cells) => *cells,
                _ => 0,
            })
            .sum();
        let column_sizes: Vec<usize> = self
            .column_sizes
            .iter()
            .map(|column_size| match column_size {
                Size::Cells(cells) => *cells,
                Size::Weight(weight) => {
                    ((columns as f64 - total_cells as f64).abs()
                        * (weight / total_weight)) as usize
                },
            })
            .collect();

        for row in self.rows.iter() {
            let mut columns_rendered_segments: Vec<
                crate::core::segment::RenderedSegments,
            > = Vec::with_capacity(row.len());

            for (column_index, component) in row.iter().enumerate() {
                columns_rendered_segments.push(component.render(
                    &crate::core::render::Options {
                        columns: Some(column_sizes[column_index]),
                        ..*options
                    },
                ));
            }

            let mut index: usize = 0;
            let mut done: usize = 0;

            while done < columns_rendered_segments.len() {
                rendered_segments.push(String::new());

                for (columns_index, column_rendered_segments) in
                    columns_rendered_segments.iter().enumerate()
                {
                    if index < column_rendered_segments.len() {
                        rendered_segments
                            .last_mut()
                            .unwrap()
                            .push_str(&column_rendered_segments[index]);
                    }
                    else {
                        rendered_segments.last_mut().unwrap().push_str(
                            &std::iter::repeat(" ")
                                .take(column_sizes[columns_index])
                                .collect::<String>(),
                        );
                    }

                    if column_rendered_segments.len() == index + 1 {
                        done += 1;
                    }
                }
                index += 1;
            }
        }

        rendered_segments
    }
}
