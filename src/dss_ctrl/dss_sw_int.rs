#[doc = "Register `DSS_SW_INT` reader"]
pub type R = crate::R<DssSwIntSpec>;
#[doc = "Register `DSS_SW_INT` writer"]
pub type W = crate::W<DssSwIntSpec>;
#[doc = "Field `dss_swint` reader - 3:0\\]
Write pulse bit field: DSS SW Interrupt Write 1 : Generate an interrupt on DSS_SW_INT0"]
pub type DssSwintR = crate::FieldReader;
#[doc = "Field `dss_swint` writer - 3:0\\]
Write pulse bit field: DSS SW Interrupt Write 1 : Generate an interrupt on DSS_SW_INT0"]
pub type DssSwintW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Write pulse bit field: DSS SW Interrupt Write 1 : Generate an interrupt on DSS_SW_INT0"]
    #[inline(always)]
    pub fn dss_swint(&self) -> DssSwintR {
        DssSwintR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Write pulse bit field: DSS SW Interrupt Write 1 : Generate an interrupt on DSS_SW_INT0"]
    #[inline(always)]
    #[must_use]
    pub fn dss_swint(&mut self) -> DssSwintW<DssSwIntSpec> {
        DssSwintW::new(self, 0)
    }
}
#[doc = "DSS_SW_INT\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_sw_int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_sw_int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssSwIntSpec;
impl crate::RegisterSpec for DssSwIntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_sw_int::R`](R) reader structure"]
impl crate::Readable for DssSwIntSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_sw_int::W`](W) writer structure"]
impl crate::Writable for DssSwIntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_SW_INT to value 0"]
impl crate::Resettable for DssSwIntSpec {
    const RESET_VALUE: u32 = 0;
}
