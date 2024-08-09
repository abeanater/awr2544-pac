#[doc = "Register `CSI2_TX_PARSTATCFG` reader"]
pub type R = crate::R<Csi2TxParstatcfgSpec>;
#[doc = "Register `CSI2_TX_PARSTATCFG` writer"]
pub type W = crate::W<Csi2TxParstatcfgSpec>;
#[doc = "Field `stat` reader - 6:0\\]
Parity address from CSI2"]
pub type StatR = crate::FieldReader;
#[doc = "Field `stat` writer - 6:0\\]
Parity address from CSI2"]
pub type StatW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `clr` reader - 8:8\\]
Clear bit for the Parity error from CSI2"]
pub type ClrR = crate::BitReader;
#[doc = "Field `clr` writer - 8:8\\]
Clear bit for the Parity error from CSI2"]
pub type ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `enable` reader - 9:9\\]
Enable bit for the Parity computation in CSI2"]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - 9:9\\]
Enable bit for the Parity computation in CSI2"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sten` reader - 10:10\\]
Enable bit for the self test of the Parity logic in CSI2"]
pub type StenR = crate::BitReader;
#[doc = "Field `sten` writer - 10:10\\]
Enable bit for the self test of the Parity logic in CSI2"]
pub type StenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Parity address from CSI2"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Clear bit for the Parity error from CSI2"]
    #[inline(always)]
    pub fn clr(&self) -> ClrR {
        ClrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable bit for the Parity computation in CSI2"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Enable bit for the self test of the Parity logic in CSI2"]
    #[inline(always)]
    pub fn sten(&self) -> StenR {
        StenR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Parity address from CSI2"]
    #[inline(always)]
    #[must_use]
    pub fn stat(&mut self) -> StatW<Csi2TxParstatcfgSpec> {
        StatW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Clear bit for the Parity error from CSI2"]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> ClrW<Csi2TxParstatcfgSpec> {
        ClrW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable bit for the Parity computation in CSI2"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<Csi2TxParstatcfgSpec> {
        EnableW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Enable bit for the self test of the Parity logic in CSI2"]
    #[inline(always)]
    #[must_use]
    pub fn sten(&mut self) -> StenW<Csi2TxParstatcfgSpec> {
        StenW::new(self, 10)
    }
}
#[doc = "CSI2_TX_PARSTATCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_tx_parstatcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_tx_parstatcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2TxParstatcfgSpec;
impl crate::RegisterSpec for Csi2TxParstatcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_tx_parstatcfg::R`](R) reader structure"]
impl crate::Readable for Csi2TxParstatcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`csi2_tx_parstatcfg::W`](W) writer structure"]
impl crate::Writable for Csi2TxParstatcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_TX_PARSTATCFG to value 0"]
impl crate::Resettable for Csi2TxParstatcfgSpec {
    const RESET_VALUE: u32 = 0;
}
