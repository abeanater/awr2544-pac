#[doc = "Register `TPTC_DBS_CFG` reader"]
pub type R = crate::R<TptcDbsCfgSpec>;
#[doc = "Register `TPTC_DBS_CFG` writer"]
pub type W = crate::W<TptcDbsCfgSpec>;
#[doc = "Field `tptc_a0` reader - 1:0\\]
Max Burst size tieoff value for TPTC A0"]
pub type TptcA0R = crate::FieldReader;
#[doc = "Field `tptc_a0` writer - 1:0\\]
Max Burst size tieoff value for TPTC A0"]
pub type TptcA0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tptc_a1` reader - 3:2\\]
Max Burst size tieoff value for TPTC A1"]
pub type TptcA1R = crate::FieldReader;
#[doc = "Field `tptc_a1` writer - 3:2\\]
Max Burst size tieoff value for TPTC A1"]
pub type TptcA1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Max Burst size tieoff value for TPTC A0"]
    #[inline(always)]
    pub fn tptc_a0(&self) -> TptcA0R {
        TptcA0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Max Burst size tieoff value for TPTC A1"]
    #[inline(always)]
    pub fn tptc_a1(&self) -> TptcA1R {
        TptcA1R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Max Burst size tieoff value for TPTC A0"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a0(&mut self) -> TptcA0W<TptcDbsCfgSpec> {
        TptcA0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Max Burst size tieoff value for TPTC A1"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a1(&mut self) -> TptcA1W<TptcDbsCfgSpec> {
        TptcA1W::new(self, 2)
    }
}
#[doc = "TPTC_DBS_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`tptc_dbs_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tptc_dbs_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TptcDbsCfgSpec;
impl crate::RegisterSpec for TptcDbsCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tptc_dbs_cfg::R`](R) reader structure"]
impl crate::Readable for TptcDbsCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`tptc_dbs_cfg::W`](W) writer structure"]
impl crate::Writable for TptcDbsCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TPTC_DBS_CFG to value 0"]
impl crate::Resettable for TptcDbsCfgSpec {
    const RESET_VALUE: u32 = 0;
}
