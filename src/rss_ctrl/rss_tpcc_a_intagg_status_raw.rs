#[doc = "Register `RSS_TPCC_A_INTAGG_STATUS_RAW` reader"]
pub type R = crate::R<RssTpccAIntaggStatusRawSpec>;
#[doc = "Register `RSS_TPCC_A_INTAGG_STATUS_RAW` writer"]
pub type W = crate::W<RssTpccAIntaggStatusRawSpec>;
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
Raw Status of Interrupt from TPTC A0. Set irrespective if the Interupt is masked or unmasked in RCSS_TPCC_A_INTAGG_MASK"]
pub type TptcA0R = crate::BitReader;
#[doc = "Field `tptc_a0` writer - 16:16\\]
Raw Status of Interrupt from TPTC A0. Set irrespective if the Interupt is masked or unmasked in RCSS_TPCC_A_INTAGG_MASK"]
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
Raw Status of Interrupt from TPTC A0. Set irrespective if the Interupt is masked or unmasked in RCSS_TPCC_A_INTAGG_MASK"]
    #[inline(always)]
    pub fn tptc_a0(&self) -> TptcA0R {
        TptcA0R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_intg(&mut self) -> TpccAIntgW<RssTpccAIntaggStatusRawSpec> {
        TpccAIntgW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_int0(&mut self) -> TpccAInt0W<RssTpccAIntaggStatusRawSpec> {
        TpccAInt0W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_int1(&mut self) -> TpccAInt1W<RssTpccAIntaggStatusRawSpec> {
        TpccAInt1W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_int2(&mut self) -> TpccAInt2W<RssTpccAIntaggStatusRawSpec> {
        TpccAInt2W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_int3(&mut self) -> TpccAInt3W<RssTpccAIntaggStatusRawSpec> {
        TpccAInt3W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_int4(&mut self) -> TpccAInt4W<RssTpccAIntaggStatusRawSpec> {
        TpccAInt4W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_int5(&mut self) -> TpccAInt5W<RssTpccAIntaggStatusRawSpec> {
        TpccAInt5W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_int6(&mut self) -> TpccAInt6W<RssTpccAIntaggStatusRawSpec> {
        TpccAInt6W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_int7(&mut self) -> TpccAInt7W<RssTpccAIntaggStatusRawSpec> {
        TpccAInt7W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Raw Status of Interrupt from TPTC A0. Set irrespective if the Interupt is masked or unmasked in RCSS_TPCC_A_INTAGG_MASK"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a0(&mut self) -> TptcA0W<RssTpccAIntaggStatusRawSpec> {
        TptcA0W::new(self, 16)
    }
}
#[doc = "RSS_TPCC_A_INTAGG_STATUS_RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_tpcc_a_intagg_status_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_tpcc_a_intagg_status_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssTpccAIntaggStatusRawSpec;
impl crate::RegisterSpec for RssTpccAIntaggStatusRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rss_tpcc_a_intagg_status_raw::R`](R) reader structure"]
impl crate::Readable for RssTpccAIntaggStatusRawSpec {}
#[doc = "`write(|w| ..)` method takes [`rss_tpcc_a_intagg_status_raw::W`](W) writer structure"]
impl crate::Writable for RssTpccAIntaggStatusRawSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSS_TPCC_A_INTAGG_STATUS_RAW to value 0"]
impl crate::Resettable for RssTpccAIntaggStatusRawSpec {
    const RESET_VALUE: u32 = 0;
}
