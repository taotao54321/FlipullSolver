use std::path::Path;

use anyhow::{ensure, Context as _};

const PRG_LEN: usize = 0x8000;

const CHR_BANK_COUNT: usize = 4;
const CHR_BANK_LEN: usize = 0x2000;
const CHR_LEN: usize = CHR_BANK_LEN * CHR_BANK_COUNT;

#[derive(Debug)]
pub struct Rom {
    prg: Box<[u8; PRG_LEN]>,
    chr: Box<[u8; CHR_LEN]>,
}

impl Rom {
    pub fn from_ines_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        Self::_from_ines_file(path.as_ref())
    }

    fn _from_ines_file(path: &Path) -> anyhow::Result<Self> {
        let ines = std::fs::read(path)
            .with_context(|| format!("ROM ファイル '{}' を読めない", path.display()))?;

        Self::_from_ines_bytes(&ines)
    }

    pub fn from_ines_bytes(ines: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        Self::_from_ines_bytes(ines.as_ref())
    }

    fn _from_ines_bytes(ines: &[u8]) -> anyhow::Result<Self> {
        const HEADER_LEN: usize = 16;

        ensure!(ines.len() >= HEADER_LEN, "iNES ヘッダの途中で EOF に達した");
        let (header, body) = ines.split_at(HEADER_LEN);

        ensure!(header.starts_with(b"NES\x1A"), "iNES magic がない");

        ensure!(body.len() >= PRG_LEN, "PRG の途中で EOF に達した");
        let (prg, chr) = body.split_at(PRG_LEN);
        ensure!(
            chr.len() == CHR_LEN,
            "CHR サイズが一致しない (expect={CHR_LEN:#06X}, actual={:#06X})",
            chr.len()
        );

        let prg: Box<[u8; PRG_LEN]> = prg.to_vec().try_into().unwrap();
        let chr: Box<[u8; CHR_LEN]> = chr.to_vec().try_into().unwrap();

        Ok(Self { prg, chr })
    }

    pub fn prg(&self) -> &[u8; PRG_LEN] {
        &self.prg
    }

    pub fn chr_bank(&self, id: usize) -> &[u8; CHR_BANK_LEN] {
        self.chr[CHR_BANK_LEN * id..][..CHR_BANK_LEN]
            .try_into()
            .unwrap()
    }
}
