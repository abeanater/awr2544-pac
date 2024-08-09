#[doc = "Register `UERRADDR1` reader"]
pub type R = crate::R<Uerraddr1Spec>;
#[doc = "Register `UERRADDR1` writer"]
pub type W = crate::W<Uerraddr1Spec>;
#[doc = "Field `UERRADDR1` reader - 10:0\\]
Uncorrectable Parity or double bit ECC error address This register holds the address of the RAM location if a parity or double bit ECC error is detected when reading the MibSPI (Receive) RXRAM. The address captured is byte alligned when RAM Parity Check is supported. This error address is frozen from being updated until it is read by the VBUS host. Reading this register clears its contents to the default value The default value is 0x400 if Extended Buffer feature is enabled, else it is 0x200 Writes to this register are ignored"]
pub type Uerraddr1R = crate::FieldReader<u16>;
#[doc = "Field `UERRADDR1` writer - 10:0\\]
Uncorrectable Parity or double bit ECC error address This register holds the address of the RAM location if a parity or double bit ECC error is detected when reading the MibSPI (Receive) RXRAM. The address captured is byte alligned when RAM Parity Check is supported. This error address is frozen from being updated until it is read by the VBUS host. Reading this register clears its contents to the default value The default value is 0x400 if Extended Buffer feature is enabled, else it is 0x200 Writes to this register are ignored"]
pub type Uerraddr1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `NU` reader - 31:11\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:11\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Uncorrectable Parity or double bit ECC error address This register holds the address of the RAM location if a parity or double bit ECC error is detected when reading the MibSPI (Receive) RXRAM. The address captured is byte alligned when RAM Parity Check is supported. This error address is frozen from being updated until it is read by the VBUS host. Reading this register clears its contents to the default value The default value is 0x400 if Extended Buffer feature is enabled, else it is 0x200 Writes to this register are ignored"]
    #[inline(always)]
    pub fn uerraddr1(&self) -> Uerraddr1R {
        Uerraddr1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Uncorrectable Parity or double bit ECC error address This register holds the address of the RAM location if a parity or double bit ECC error is detected when reading the MibSPI (Receive) RXRAM. The address captured is byte alligned when RAM Parity Check is supported. This error address is frozen from being updated until it is read by the VBUS host. Reading this register clears its contents to the default value The default value is 0x400 if Extended Buffer feature is enabled, else it is 0x200 Writes to this register are ignored"]
    #[inline(always)]
    #[must_use]
    pub fn uerraddr1(&mut self) -> Uerraddr1W<Uerraddr1Spec> {
        Uerraddr1W::new(self, 0)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Uerraddr1Spec> {
        NuW::new(self, 11)
    }
}
#[doc = "Uncorrectable Parity or double bit ECC error Address Register - RXRAM\n\nYou can [`read`](crate::Reg::read) this register and get [`uerraddr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uerraddr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uerraddr1Spec;
impl crate::RegisterSpec for Uerraddr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uerraddr1::R`](R) reader structure"]
impl crate::Readable for Uerraddr1Spec {}
#[doc = "`write(|w| ..)` method takes [`uerraddr1::W`](W) writer structure"]
impl crate::Writable for Uerraddr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UERRADDR1 to value 0"]
impl crate::Resettable for Uerraddr1Spec {
    const RESET_VALUE: u32 = 0;
}
