use crate::commons::editor::TextEditor;

#[derive(Default)]
pub struct MetaHeaders {
    headers: Vec<KV>,
}

impl MetaHeaders {
    pub(crate) fn add(&mut self) {
        self.headers.push(KV::default())
    }

    pub(crate) fn remove(&mut self, index: usize) {
        self.headers.remove(index);
    }

    pub(crate) fn headers(&self) -> Vec<(&TextEditor, &TextEditor)> {
        self.headers.iter().map(|x| (&x.key, &x.val)).collect()
    }
}

#[derive(Default)]
struct KV {
    key: TextEditor,
    val: TextEditor,
}
