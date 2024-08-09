#[doc = "Register `ECC_reserved_svbus_2` reader"]
pub type R = crate::R<EccReservedSvbus2Spec>;
#[doc = "Register `ECC_reserved_svbus_2` writer"]
pub type W = crate::W<EccReservedSvbus2Spec>;
#[doc = "Field `SERIAL_VBUS_REGISTER` reader - 31:0\\]
Serial VBUS register data"]
pub type SerialVbusRegisterR = crate::FieldReader<u32>;
#[doc = "Field `SERIAL_VBUS_REGISTER` writer - 31:0\\]
Serial VBUS register data"]
pub type SerialVbusRegisterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Serial VBUS register data"]
    #[inline(always)]
    pub fn serial_vbus_register(&self) -> SerialVbusRegisterR {
        SerialVbusRegisterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Serial VBUS register data"]
    #[inline(always)]
    #[must_use]
    pub fn serial_vbus_register(&mut self) -> SerialVbusRegisterW<EccReservedSvbus2Spec> {
        SerialVbusRegisterW::new(self, 0)
    }
}
#[doc = "Reference other documents that contain the ECC RAM wrapper and EDC controller serial vbus register sets.\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_reserved_svbus_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_reserved_svbus_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccReservedSvbus2Spec;
impl crate::RegisterSpec for EccReservedSvbus2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_reserved_svbus_2::R`](R) reader structure"]
impl crate::Readable for EccReservedSvbus2Spec {}
#[doc = "`write(|w| ..)` method takes [`ecc_reserved_svbus_2::W`](W) writer structure"]
impl crate::Writable for EccReservedSvbus2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECC_reserved_svbus_2 to value 0"]
impl crate::Resettable for EccReservedSvbus2Spec {
    const RESET_VALUE: u32 = 0;
}
