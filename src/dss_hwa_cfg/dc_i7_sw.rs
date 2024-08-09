#[doc = "Register `DC_I7_SW` reader"]
pub type R = crate::R<DcI7SwSpec>;
#[doc = "Register `DC_I7_SW` writer"]
pub type W = crate::W<DcI7SwSpec>;
#[doc = "Field `dc_i7_sw` reader - 23:0\\]
SW programmed DC I value(for bcnt =7 ) used in DC subtraction"]
pub type DcI7SwR = crate::FieldReader<u32>;
#[doc = "Field `dc_i7_sw` writer - 23:0\\]
SW programmed DC I value(for bcnt =7 ) used in DC subtraction"]
pub type DcI7SwW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
SW programmed DC I value(for bcnt =7 ) used in DC subtraction"]
    #[inline(always)]
    pub fn dc_i7_sw(&self) -> DcI7SwR {
        DcI7SwR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
SW programmed DC I value(for bcnt =7 ) used in DC subtraction"]
    #[inline(always)]
    #[must_use]
    pub fn dc_i7_sw(&mut self) -> DcI7SwW<DcI7SwSpec> {
        DcI7SwW::new(self, 0)
    }
}
#[doc = "DC_I7_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_i7_sw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_i7_sw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcI7SwSpec;
impl crate::RegisterSpec for DcI7SwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_i7_sw::R`](R) reader structure"]
impl crate::Readable for DcI7SwSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_i7_sw::W`](W) writer structure"]
impl crate::Writable for DcI7SwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_I7_SW to value 0"]
impl crate::Resettable for DcI7SwSpec {
    const RESET_VALUE: u32 = 0;
}
