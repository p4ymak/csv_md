use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct CsvTable {
    separator: char,
    pub header: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl Default for CsvTable {
    fn default() -> Self {
        CsvTable {
            separator: ',',
            header: vec![],
            rows: vec![],
        }
    }
}

impl CsvTable {
    pub fn new(separator: char, text: &str) -> Self {
        let mut table = CsvTable {
            separator,
            ..Default::default()
        };
        table.push_rows(text);
        table
    }

    pub fn parse_row(row: &str, separator: char) -> Vec<String> {
        row.split(separator).map(|v| v.trim().to_string()).collect()
    }

    pub fn push_rows(&mut self, rows: &str) {
        for row in rows.lines().filter(|r| !r.trim().is_empty()) {
            if self.header.is_empty() {
                self.header = Self::parse_row(row, self.separator);
            } else {
                self.push_vec(Self::parse_row(row, self.separator));
            }
        }
    }

    pub fn push_vecs(&mut self, vecs: Vec<Vec<String>>) {
        for v in vecs {
            if self.header.is_empty() {
                self.header = v;
            } else {
                self.push_vec(v);
            }
        }
    }

    fn push_vec(&mut self, vec: Vec<String>) {
        let h_cols = self.header.len();
        let r_cols = vec.len();
        let mut vec = vec;
        if h_cols > r_cols {
            vec.append(&mut vec![String::new(); h_cols - r_cols]);
        } else if h_cols < r_cols {
            vec = vec[..h_cols].to_vec();
        }
        self.rows.push(vec);
    }

    pub fn is_empty(&self) -> bool {
        self.header.is_empty()
    }

    pub fn rows_count(&self) -> usize {
        self.rows.len()
    }

    pub fn header(&self) -> Vec<&str> {
        self.header.iter().map(|c| c.as_str()).collect()
    }

    pub fn header_mut(&mut self) -> Vec<&mut String> {
        self.header.iter_mut().collect()
    }

    pub fn separator(&self) -> char {
        self.separator
    }

    pub fn rows(&self) -> Vec<Vec<&str>> {
        self.rows
            .iter()
            .map(|r| r.iter().map(|v| v.as_str()).collect())
            .collect()
    }

    pub fn as_md(&self) -> String {
        table_to_markdown(self.header(), self.rows())
    }

    pub fn range_as_md(&self, preview_range: RangeInclusive<usize>) -> String {
        table_to_markdown(
            &self.header,
            self.rows
                .get(
                    *preview_range.start()
                        ..=(*preview_range
                            .end()
                            .min(&self.rows_count().saturating_sub(1))),
                )
                .unwrap_or_default(),
        )
    }
}

pub fn table_to_markdown<I, J, S>(header: J, data: I) -> String
where
    I: IntoIterator<Item = J>,
    J: IntoIterator<Item = S>,
    S: ToString,
{
    let fill = |inp: &str, size: usize| {
        let len = inp.chars().count();
        if size > len {
            let f = size.saturating_sub(len);
            format!("{}{}", inp, " ".repeat(f))
        } else {
            inp.to_owned()
        }
    };

    let mut lens = Vec::<usize>::new();
    let strings = data
        .into_iter()
        .map(|row| {
            row.into_iter()
                .enumerate()
                .map(|(i, v)| {
                    let mut s = v.to_string();
                    let len = s.chars().count();
                    if lens.len() <= i {
                        lens.push(len);
                    }
                    if let Some(l) = lens.get_mut(i) {
                        *l = s.chars().count().max(*l);
                        s = fill(&s, *l);
                    }
                    s
                })
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let header = header
        .into_iter()
        .enumerate()
        .map(|(i, s)| {
            let mut st = s.to_string();
            if let Some(l) = lens.get_mut(i) {
                *l = st.chars().count().max(*l);
                st = fill(&st, *l);
            }

            fill(&st, *lens.get(i).unwrap_or(&0))
        })
        .collect::<Vec<String>>();

    let divider = lens.iter().map(|l| "-".repeat(*l)).collect::<Vec<String>>();

    [header, divider]
        .into_iter()
        .chain(strings)
        .map(|row| {
            let mut first = vec![String::new()];
            first.extend(
                row.iter()
                    .enumerate()
                    .map(|(i, s)| fill(s, *lens.get(i).unwrap_or(&0)))
                    .collect::<Vec<String>>(),
            );
            first.push(String::new());
            first.join(" | ")
        })
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
pub mod test {
    use crate::CsvTable;

    #[test]
    fn about() {
        let input = "ID, Name, Description
1, Language, Rust
2, CSV_MD, Convert CSV to MD
,
3, License, MIT, Use for your own, risk
4, Author, Roman Chumak";

        let output = " | ID | Name     | Description       | 
 | -- | -------- | ----------------- | 
 | 1  | Language | Rust              | 
 | 2  | CSV_MD   | Convert CSV to MD | 
 |    |          |                   | 
 | 3  | License  | MIT               | 
 | 4  | Author   | Roman Chumak      | ";
        assert_eq!(CsvTable::new(',', input).as_md(), output.to_string());
    }
}
