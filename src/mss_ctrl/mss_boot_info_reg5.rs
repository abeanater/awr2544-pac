#[doc = "Register `MSS_BOOT_INFO_REG5` reader"]
pub type R = crate::R<MssBootInfoReg5Spec>;
#[doc = "Register `MSS_BOOT_INFO_REG5` writer"]
pub type W = crate::W<MssBootInfoReg5Spec>;
#[doc = "Field `config` reader - 31:0\\]
Reserved Register for Software use"]
pub type ConfigR = crate::FieldReader<u32>;
#[doc = "Field `config` writer - 31:0\\]
Reserved Register for Software use"]
pub type ConfigW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved Register for Software use"]
    #[inline(always)]
    pub fn config(&self) -> ConfigR {
        ConfigR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved Register for Software use"]
    #[inline(always)]
    #[must_use]
    pub fn config(&mut self) -> ConfigW<MssBootInfoReg5Spec> {
        ConfigW::new(self, 0)
    }
}
#[doc = "MSS_BOOT_INFO_REG5\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_boot_info_reg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_boot_info_reg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssBootInfoReg5Spec;
impl crate::RegisterSpec for MssBootInfoReg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_boot_info_reg5::R`](R) reader structure"]
impl crate::Readable for MssBootInfoReg5Spec {}
#[doc = "`write(|w| ..)` method takes [`mss_boot_info_reg5::W`](W) writer structure"]
impl crate::Writable for MssBootInfoReg5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_BOOT_INFO_REG5 to value 0"]
impl crate::Resettable for MssBootInfoReg5Spec {
    const RESET_VALUE: u32 = 0;
}
