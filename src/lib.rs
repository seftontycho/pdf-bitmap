use pyo3::prelude::*;

const JPG_START: [u8; 2] = [0xFF, 0xD8];
const JPG_END: [u8; 2] = [0xFF, 0xD9];
const STR_START: &[u8] = "stream".as_bytes();
const STR_END: &[u8] = "endstream".as_bytes();

fn jpgs_from_stream(mut data: &[u8]) -> Vec<Vec<u8>> {
    let mut jpgs = Vec::new();

    while let Some(pos) = data
        .windows(JPG_START.len())
        .position(|window| window == JPG_START)
    {
        let end = match data[pos + JPG_START.len()..]
            .windows(JPG_END.len())
            .position(|window| window == JPG_END)
        {
            Some(end) => end,
            None => break,
        };

        let jpg = data[pos..pos + JPG_START.len() + end + JPG_END.len()].to_vec();
        jpgs.push(jpg);

        data = &data[pos + JPG_START.len() + end + JPG_END.len()..];
    }

    jpgs
}

pub fn extract_jpgs(mut data: &[u8]) -> Vec<Vec<u8>> {
    let mut jpgs = Vec::new();

    while let Some(stream_start) = data
        .windows(STR_START.len())
        .position(|window| window == STR_START)
    {
        let stream_start = stream_start + STR_START.len();
        let stream_length = match data[stream_start..]
            .windows(STR_END.len())
            .position(|window| window == STR_END)
        {
            Some(pos) => pos,
            None => break,
        };

        let stream = &data[stream_start..stream_start + stream_length];
        let found = jpgs_from_stream(stream);

        jpgs.extend(found);
        data = &data[stream_start + stream_length + STR_END.len()..];
    }

    jpgs
}

#[pyfunction]
fn pdf_2_jpgs(pdf: &[u8]) -> PyResult<Vec<Vec<u8>>> {
    let jpgs = extract_jpgs(pdf);
    Ok(jpgs)
}

#[pymodule]
pub fn pdf_bitmap(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pdf_2_jpgs, m)?)?;
    Ok(())
}
