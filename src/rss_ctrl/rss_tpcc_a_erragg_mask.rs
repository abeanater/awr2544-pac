#[doc = "Register `RSS_TPCC_A_ERRAGG_MASK` reader"]
pub type R = crate::R<RssTpccAErraggMaskSpec>;
#[doc = "Register `RSS_TPCC_A_ERRAGG_MASK` writer"]
pub type W = crate::W<RssTpccAErraggMaskSpec>;
#[doc = "Field `tpcc_a_errint` reader - "]
pub type TpccAErrintR = crate::BitReader;
#[doc = "Field `tpcc_a_errint` writer - "]
pub type TpccAErrintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_a_mpint` reader - "]
pub type TpccAMpintR = crate::BitReader;
#[doc = "Field `tpcc_a_mpint` writer - "]
pub type TpccAMpintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_a0_err` reader - "]
pub type TptcA0ErrR = crate::BitReader;
#[doc = "Field `tptc_a0_err` writer - "]
pub type TptcA0ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_a_parity_err` reader - "]
pub type TpccAParityErrR = crate::BitReader;
#[doc = "Field `tpcc_a_parity_err` writer - "]
pub type TpccAParityErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_a_write_access_error` reader - "]
pub type TpccAWriteAccessErrorR = crate::BitReader;
#[doc = "Field `tpcc_a_write_access_error` writer - "]
pub type TpccAWriteAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_a0_write_access_error` reader - "]
pub type TptcA0WriteAccessErrorR = crate::BitReader;
#[doc = "Field `tptc_a0_write_access_error` writer - "]
pub type TptcA0WriteAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_a_read_access_error` reader - "]
pub type TpccAReadAccessErrorR = crate::BitReader;
#[doc = "Field `tpcc_a_read_access_error` writer - "]
pub type TpccAReadAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_a0_read_access_error` reader - "]
pub type TptcA0ReadAccessErrorR = crate::BitReader;
#[doc = "Field `tptc_a0_read_access_error` writer - "]
pub type TptcA0ReadAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tpcc_a_errint(&self) -> TpccAErrintR {
        TpccAErrintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tpcc_a_mpint(&self) -> TpccAMpintR {
        TpccAMpintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tptc_a0_err(&self) -> TptcA0ErrR {
        TptcA0ErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tpcc_a_parity_err(&self) -> TpccAParityErrR {
        TpccAParityErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tpcc_a_write_access_error(&self) -> TpccAWriteAccessErrorR {
        TpccAWriteAccessErrorR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tptc_a0_write_access_error(&self) -> TptcA0WriteAccessErrorR {
        TptcA0WriteAccessErrorR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tpcc_a_read_access_error(&self) -> TpccAReadAccessErrorR {
        TpccAReadAccessErrorR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tptc_a0_read_access_error(&self) -> TptcA0ReadAccessErrorR {
        TptcA0ReadAccessErrorR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_errint(&mut self) -> TpccAErrintW<RssTpccAErraggMaskSpec> {
        TpccAErrintW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_mpint(&mut self) -> TpccAMpintW<RssTpccAErraggMaskSpec> {
        TpccAMpintW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a0_err(&mut self) -> TptcA0ErrW<RssTpccAErraggMaskSpec> {
        TptcA0ErrW::new(self, 2)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_parity_err(&mut self) -> TpccAParityErrW<RssTpccAErraggMaskSpec> {
        TpccAParityErrW::new(self, 8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_write_access_error(&mut self) -> TpccAWriteAccessErrorW<RssTpccAErraggMaskSpec> {
        TpccAWriteAccessErrorW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a0_write_access_error(
        &mut self,
    ) -> TptcA0WriteAccessErrorW<RssTpccAErraggMaskSpec> {
        TptcA0WriteAccessErrorW::new(self, 17)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_read_access_error(&mut self) -> TpccAReadAccessErrorW<RssTpccAErraggMaskSpec> {
        TpccAReadAccessErrorW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a0_read_access_error(&mut self) -> TptcA0ReadAccessErrorW<RssTpccAErraggMaskSpec> {
        TptcA0ReadAccessErrorW::new(self, 25)
    }
}
#[doc = "RSS_TPCC_A_ERRAGG_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_tpcc_a_erragg_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_tpcc_a_erragg_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssTpccAErraggMaskSpec;
impl crate::RegisterSpec for RssTpccAErraggMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rss_tpcc_a_erragg_mask::R`](R) reader structure"]
impl crate::Readable for RssTpccAErraggMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`rss_tpcc_a_erragg_mask::W`](W) writer structure"]
impl crate::Writable for RssTpccAErraggMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSS_TPCC_A_ERRAGG_MASK to value 0"]
impl crate::Resettable for RssTpccAErraggMaskSpec {
    const RESET_VALUE: u32 = 0;
}
