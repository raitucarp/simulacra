//! Vector arithmetic and the BLOB encoding (`f32`, little-endian, packed).

/// Bytes per stored component.
const F32_BYTES: usize = 4;

/// `vector` scaled to unit length (a zero vector stays zero).
pub fn l2_normalized(vector: Vec<f32>) -> Vec<f32> {
    let norm = dot(&vector, &vector).sqrt();
    if norm <= f32::EPSILON {
        return vector;
    }
    vector.into_iter().map(|x| x / norm).collect()
}

/// The dot product over the shared length.
pub fn dot(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

/// Cosine similarity in `[-1, 1]`; 0 when either vector is zero or the lengths differ.
pub fn cosine(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }
    let norms = dot(a, a).sqrt() * dot(b, b).sqrt();
    if norms <= f32::EPSILON {
        return 0.0;
    }
    dot(a, b) / norms
}

/// The component-wise mean of same-length vectors (`None` when empty or ragged).
pub fn mean_vector(vectors: &[&[f32]]) -> Option<Vec<f32>> {
    let first = vectors.first()?;
    if vectors.iter().any(|v| v.len() != first.len()) {
        return None;
    }
    let count = vectors.len() as f32;
    Some((0..first.len()).map(|i| vectors.iter().map(|v| v[i]).sum::<f32>() / count).collect())
}

/// Packs `vector` as little-endian `f32`s.
pub fn encode_f32_le(vector: &[f32]) -> Vec<u8> {
    vector.iter().flat_map(|x| x.to_le_bytes()).collect()
}

/// Unpacks a little-endian `f32` blob (`None` when its length is not a multiple of 4).
pub fn decode_f32_le(bytes: &[u8]) -> Option<Vec<f32>> {
    if bytes.len() % F32_BYTES != 0 {
        return None;
    }
    let floats = bytes.chunks_exact(F32_BYTES).map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]));
    Some(floats.collect())
}
