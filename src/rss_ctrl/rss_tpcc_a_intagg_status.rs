#[doc = "Register `RSS_TPCC_A_INTAGG_STATUS` reader"]
pub type R = crate::R<RssTpccAIntaggStatusSpec>;
#[doc = "Register `RSS_TPCC_A_INTAGG_STATUS` writer"]
pub type W = crate::W<RssTpccAIntaggStatusSpec>;
#[doc = "Field `tpcc_a_intg` reader - "]
pub type TpccAIntgR = crate::BitReader;
#[doc = "Field `tpcc_a_intg` writer - "]
pub type TpccAIntgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_a_int0` reader - "]
pub type TpccAInt0R = crate::BitReader;
#[doc = "Field `tpcc_a_int0` writer - "]
pub type TpccAInt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_a_int1` reader - "]
pub type TpccAInt1R = crate::BitReader;
#[doc = "Field `tpcc_a_int1` writer - "]
pub type TpccAInt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_a_int2` reader - "]
pub type TpccAInt2R = crate::BitReader;
#[doc = "Field `tpcc_a_int2` writer - "]
pub type TpccAInt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_a_int3` reader - "]
pub type TpccAInt3R = crate::BitReader;
#[doc = "Field `tpcc_a_int3` writer - "]
pub type TpccAInt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_a_int4` reader - "]
pub type TpccAInt4R = crate::BitReader;
#[doc = "Field `tpcc_a_int4` writer - "]
pub type TpccAInt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_a_int5` reader - "]
pub type TpccAInt5R = crate::BitReader;
#[doc = "Field `tpcc_a_int5` writer - "]
pub type TpccAInt5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_a_int6` reader - "]
pub type TpccAInt6R = crate::BitReader;
#[doc = "Field `tpcc_a_int6` writer - "]
pub type TpccAInt6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_a_int7` reader - "]
pub type TpccAInt7R = crate::BitReader;
#[doc = "Field `tpcc_a_int7` writer - "]
pub type TpccAInt7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_a0` reader - 16:16\\]
Status of Interrupt from TPTC A0. Set only if Interupt is unmasked in RCSS_TPCC_A_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type TptcA0R = crate::BitReader;
#[doc = "Field `tptc_a0` writer - 16:16\\]
Status of Interrupt from TPTC A0. Set only if Interupt is unmasked in RCSS_TPCC_A_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type TptcA0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tpcc_a_intg(&self) -> TpccAIntgR {
        TpccAIntgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tpcc_a_int0(&self) -> TpccAInt0R {
        TpccAInt0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tpcc_a_int1(&self) -> TpccAInt1R {
        TpccAInt1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tpcc_a_int2(&self) -> TpccAInt2R {
        TpccAInt2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tpcc_a_int3(&self) -> TpccAInt3R {
        TpccAInt3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tpcc_a_int4(&self) -> TpccAInt4R {
        TpccAInt4R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tpcc_a_int5(&self) -> TpccAInt5R {
        TpccAInt5R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tpcc_a_int6(&self) -> TpccAInt6R {
        TpccAInt6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tpcc_a_int7(&self) -> TpccAInt7R {
        TpccAInt7R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Status of Interrupt from TPTC A0. Set only if Interupt is unmasked in RCSS_TPCC_A_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn tptc_a0(&self) -> TptcA0R {
        TptcA0R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_intg(&mut self) -> TpccAIntgW<RssTpccAIntaggStatusSpec> {
        TpccAIntgW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_int0(&mut self) -> TpccAInt0W<RssTpccAIntaggStatusSpec> {
        TpccAInt0W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_int1(&mut self) -> TpccAInt1W<RssTpccAIntaggStatusSpec> {
        TpccAInt1W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_int2(&mut self) -> TpccAInt2W<RssTpccAIntaggStatusSpec> {
        TpccAInt2W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_int3(&mut self) -> TpccAInt3W<RssTpccAIntaggStatusSpec> {
        TpccAInt3W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_int4(&mut self) -> TpccAInt4W<RssTpccAIntaggStatusSpec> {
        TpccAInt4W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_int5(&mut self) -> TpccAInt5W<RssTpccAIntaggStatusSpec> {
        TpccAInt5W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_int6(&mut self) -> TpccAInt6W<RssTpccAIntaggStatusSpec> {
        TpccAInt6W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_int7(&mut self) -> TpccAInt7W<RssTpccAIntaggStatusSpec> {
        TpccAInt7W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Status of Interrupt from TPTC A0. Set only if Interupt is unmasked in RCSS_TPCC_A_INTAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a0(&mut self) -> TptcA0W<RssTpccAIntaggStatusSpec> {
        TptcA0W::new(self, 16)
    }
}
#[doc = "RSS_TPCC_A_INTAGG_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_tpcc_a_intagg_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_tpcc_a_intagg_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssTpccAIntaggStatusSpec;
impl crate::RegisterSpec for RssTpccAIntaggStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rss_tpcc_a_intagg_status::R`](R) reader structure"]
impl crate::Readable for RssTpccAIntaggStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`rss_tpcc_a_intagg_status::W`](W) writer structure"]
impl crate::Writable for RssTpccAIntaggStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSS_TPCC_A_INTAGG_STATUS to value 0"]
impl crate::Resettable for RssTpccAIntaggStatusSpec {
    const RESET_VALUE: u32 = 0;
}
