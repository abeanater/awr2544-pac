#[doc = "Register `MSS_CAPEVNT_SEL` reader"]
pub type R = crate::R<MssCapevntSelSpec>;
#[doc = "Register `MSS_CAPEVNT_SEL` writer"]
pub type W = crate::W<MssCapevntSelSpec>;
#[doc = "Field `src0` reader - 7:0\\]
Writing a value 'N' will select Nth interrupt from CR5A/B interrupt mapping to trigger CAP-EVENT0 to all MSS_RTIs. Example: writing 8'h0A will select 10th interrupt to trigger CAP-EVENT0 to all MSS_RTIs. (which is MSS_RTIB_INT1)"]
pub type Src0R = crate::FieldReader;
#[doc = "Field `src0` writer - 7:0\\]
Writing a value 'N' will select Nth interrupt from CR5A/B interrupt mapping to trigger CAP-EVENT0 to all MSS_RTIs. Example: writing 8'h0A will select 10th interrupt to trigger CAP-EVENT0 to all MSS_RTIs. (which is MSS_RTIB_INT1)"]
pub type Src0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `src1` reader - 15:8\\]
Writing a value 'N' will select Nth interrupt from CR5A/B interrupt mapping to trigger CAP-EVENT1 to all MSS_RTIs. Example: writing 8'h0A will select 10th interrupt to trigger CAP-EVENT1 to all MSS_RTIs. (which is MSS_RTIB_INT1)"]
pub type Src1R = crate::FieldReader;
#[doc = "Field `src1` writer - 15:8\\]
Writing a value 'N' will select Nth interrupt from CR5A/B interrupt mapping to trigger CAP-EVENT1 to all MSS_RTIs. Example: writing 8'h0A will select 10th interrupt to trigger CAP-EVENT1 to all MSS_RTIs. (which is MSS_RTIB_INT1)"]
pub type Src1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Writing a value 'N' will select Nth interrupt from CR5A/B interrupt mapping to trigger CAP-EVENT0 to all MSS_RTIs. Example: writing 8'h0A will select 10th interrupt to trigger CAP-EVENT0 to all MSS_RTIs. (which is MSS_RTIB_INT1)"]
    #[inline(always)]
    pub fn src0(&self) -> Src0R {
        Src0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Writing a value 'N' will select Nth interrupt from CR5A/B interrupt mapping to trigger CAP-EVENT1 to all MSS_RTIs. Example: writing 8'h0A will select 10th interrupt to trigger CAP-EVENT1 to all MSS_RTIs. (which is MSS_RTIB_INT1)"]
    #[inline(always)]
    pub fn src1(&self) -> Src1R {
        Src1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Writing a value 'N' will select Nth interrupt from CR5A/B interrupt mapping to trigger CAP-EVENT0 to all MSS_RTIs. Example: writing 8'h0A will select 10th interrupt to trigger CAP-EVENT0 to all MSS_RTIs. (which is MSS_RTIB_INT1)"]
    #[inline(always)]
    #[must_use]
    pub fn src0(&mut self) -> Src0W<MssCapevntSelSpec> {
        Src0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Writing a value 'N' will select Nth interrupt from CR5A/B interrupt mapping to trigger CAP-EVENT1 to all MSS_RTIs. Example: writing 8'h0A will select 10th interrupt to trigger CAP-EVENT1 to all MSS_RTIs. (which is MSS_RTIB_INT1)"]
    #[inline(always)]
    #[must_use]
    pub fn src1(&mut self) -> Src1W<MssCapevntSelSpec> {
        Src1W::new(self, 8)
    }
}
#[doc = "MSS_CAPEVNT_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_capevnt_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_capevnt_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssCapevntSelSpec;
impl crate::RegisterSpec for MssCapevntSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_capevnt_sel::R`](R) reader structure"]
impl crate::Readable for MssCapevntSelSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_capevnt_sel::W`](W) writer structure"]
impl crate::Writable for MssCapevntSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_CAPEVNT_SEL to value 0"]
impl crate::Resettable for MssCapevntSelSpec {
    const RESET_VALUE: u32 = 0;
}
