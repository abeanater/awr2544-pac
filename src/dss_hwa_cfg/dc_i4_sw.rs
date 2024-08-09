#[doc = "Register `DC_I4_SW` reader"]
pub type R = crate::R<DcI4SwSpec>;
#[doc = "Register `DC_I4_SW` writer"]
pub type W = crate::W<DcI4SwSpec>;
#[doc = "Field `dc_i4_sw` reader - 23:0\\]
SW programmed DC I value(for bcnt =4 ) used in DC subtraction"]
pub type DcI4SwR = crate::FieldReader<u32>;
#[doc = "Field `dc_i4_sw` writer - 23:0\\]
SW programmed DC I value(for bcnt =4 ) used in DC subtraction"]
pub type DcI4SwW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
SW programmed DC I value(for bcnt =4 ) used in DC subtraction"]
    #[inline(always)]
    pub fn dc_i4_sw(&self) -> DcI4SwR {
        DcI4SwR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
SW programmed DC I value(for bcnt =4 ) used in DC subtraction"]
    #[inline(always)]
    #[must_use]
    pub fn dc_i4_sw(&mut self) -> DcI4SwW<DcI4SwSpec> {
        DcI4SwW::new(self, 0)
    }
}
#[doc = "DC_I4_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_i4_sw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_i4_sw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcI4SwSpec;
impl crate::RegisterSpec for DcI4SwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_i4_sw::R`](R) reader structure"]
impl crate::Readable for DcI4SwSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_i4_sw::W`](W) writer structure"]
impl crate::Writable for DcI4SwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_I4_SW to value 0"]
impl crate::Resettable for DcI4SwSpec {
    const RESET_VALUE: u32 = 0;
}
