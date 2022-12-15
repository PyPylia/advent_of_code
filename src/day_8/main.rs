pub mod viewer;

use crate::viewer::Viewer;

static INPUT: &str = include_str!("input.txt");

fn main() -> anyhow::Result<()> {
    let mut viewer = Viewer::new(INPUT);

    for y in 0..viewer.height() {
        let mut current_num = viewer.get_height(0, y);
        viewer.set_visible(0, y);

        for x in 1..viewer.width() {
            let next_num = viewer.get_height(x, y);
            if next_num > current_num {
                current_num = next_num;
                viewer.set_visible(x, y);
            }
        }

        let mut current_num = viewer.get_height(viewer.width() - 1, y);
        viewer.set_visible(viewer.width() - 1, y);

        for x in (0..viewer.width() - 1).rev() {
            let next_num = viewer.get_height(x, y);
            if next_num > current_num {
                current_num = next_num;
                viewer.set_visible(x, y);
            }
        }
    }

    for x in 0..viewer.width() {
        let mut current_num = viewer.get_height(x, 0);
        viewer.set_visible(x, 0);

        for y in 1..viewer.height() {
            let next_num = viewer.get_height(x, y);
            if next_num > current_num {
                current_num = next_num;
                viewer.set_visible(x, y);
            }
        }

        let mut current_num = viewer.get_height(x, viewer.height() - 1);
        viewer.set_visible(x, viewer.height() - 1);

        for y in (0..viewer.height() - 1).rev() {
            let next_num = viewer.get_height(x, y);
            if next_num > current_num {
                current_num = next_num;
                viewer.set_visible(x, y);
            }
        }
    }

    println!(
        "Total visible trees: {}",
        viewer.count_visible()
    );
    let mut scenic_score: usize = 0;

    for x in 0..viewer.width() {
        for y in 0..viewer.height() {
            let node = viewer.get_height(x, y);

            let mut east = 0;
            for nx in x + 1..viewer.width() {
                let next_node = viewer.get_height(nx, y);
                east += 1;
                if next_node >= node {
                    break;
                }
            }

            let mut west = 0;
            for nx in (0..x).rev() {
                let next_node = viewer.get_height(nx, y);
                west += 1;
                if next_node >= node {
                    break;
                }
            }

            let mut north = 0;
            for ny in y + 1..viewer.height() {
                let next_node = viewer.get_height(x, ny);
                north += 1;
                if next_node >= node {
                    break;
                }
            }

            let mut south = 0;
            for ny in (0..y).rev() {
                let next_node = viewer.get_height(x, ny);
                south += 1;
                if next_node >= node {
                    break;
                }
            }

            let new_scenic_score = north * east * south * west;

            if new_scenic_score > scenic_score {
                scenic_score = new_scenic_score;
            }
        }
    }

    println!(
        "Highest possible scenic score: {}",
        scenic_score
    );

    Ok(())
}
