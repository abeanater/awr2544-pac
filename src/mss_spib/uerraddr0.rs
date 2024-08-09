#[doc = "Register `UERRADDR0` reader"]
pub type R = crate::R<Uerraddr0Spec>;
#[doc = "Register `UERRADDR0` writer"]
pub type W = crate::W<Uerraddr0Spec>;
#[doc = "Field `UERRADDR0` reader - 10:0\\]
Uncorrectable Parity or double bit ECC error address This register holds the address when a parity error is generated while reading the MibSPI (Transmit) TXRAM. The TXRAM can be read either by CPU or by the MibSPI Sequencer FSM logic for transmission. The address captured is byte alligned. This error address is frozen from being updated until it is read by the VBUSP host. Reading this register clears its contents to the default value of 0x000. Writes to this register are ignored."]
pub type Uerraddr0R = crate::FieldReader<u16>;
#[doc = "Field `UERRADDR0` writer - 10:0\\]
Uncorrectable Parity or double bit ECC error address This register holds the address when a parity error is generated while reading the MibSPI (Transmit) TXRAM. The TXRAM can be read either by CPU or by the MibSPI Sequencer FSM logic for transmission. The address captured is byte alligned. This error address is frozen from being updated until it is read by the VBUSP host. Reading this register clears its contents to the default value of 0x000. Writes to this register are ignored."]
pub type Uerraddr0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `NU` reader - 31:11\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:11\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Uncorrectable Parity or double bit ECC error address This register holds the address when a parity error is generated while reading the MibSPI (Transmit) TXRAM. The TXRAM can be read either by CPU or by the MibSPI Sequencer FSM logic for transmission. The address captured is byte alligned. This error address is frozen from being updated until it is read by the VBUSP host. Reading this register clears its contents to the default value of 0x000. Writes to this register are ignored."]
    #[inline(always)]
    pub fn uerraddr0(&self) -> Uerraddr0R {
        Uerraddr0R::new((self.bits & 0x07ff) as u16)
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
Uncorrectable Parity or double bit ECC error address This register holds the address when a parity error is generated while reading the MibSPI (Transmit) TXRAM. The TXRAM can be read either by CPU or by the MibSPI Sequencer FSM logic for transmission. The address captured is byte alligned. This error address is frozen from being updated until it is read by the VBUSP host. Reading this register clears its contents to the default value of 0x000. Writes to this register are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn uerraddr0(&mut self) -> Uerraddr0W<Uerraddr0Spec> {
        Uerraddr0W::new(self, 0)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Uerraddr0Spec> {
        NuW::new(self, 11)
    }
}
#[doc = "Uncorrectable Parity or double bit ECC error address register - TXRAM\n\nYou can [`read`](crate::Reg::read) this register and get [`uerraddr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uerraddr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uerraddr0Spec;
impl crate::RegisterSpec for Uerraddr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uerraddr0::R`](R) reader structure"]
impl crate::Readable for Uerraddr0Spec {}
#[doc = "`write(|w| ..)` method takes [`uerraddr0::W`](W) writer structure"]
impl crate::Writable for Uerraddr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UERRADDR0 to value 0"]
impl crate::Resettable for Uerraddr0Spec {
    const RESET_VALUE: u32 = 0;
}
