pub enum Size {
    Cells(usize),
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
        let mut rendered_segments = Vec::new();

        let column_sizes: Vec<usize> = self
            .column_sizes
            .iter()
            .map(|column_size| match column_size {
                Size::Cells(cells) => *cells,
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
