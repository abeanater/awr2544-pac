#[doc = "Register `SBERRADDR0` reader"]
pub type R = crate::R<Sberraddr0Spec>;
#[doc = "Register `SBERRADDR0` writer"]
pub type W = crate::W<Sberraddr0Spec>;
#[doc = "Field `SBERRADDR0` reader - 10:0\\]
Single Bit ECC Error Address This register holds the address when a single bit error is generated from SECDED block while reading the MibSPI (Transmit) TXRAM. The TXRAM can be read either by CPU or by the MibSPI Sequencer logic for transmission. This error address is frozen from being updated until it is read by the VBUSP host. Reading this register clears its contents to the default value of 0x000. Writes to this register are ignored."]
pub type Sberraddr0R = crate::FieldReader<u16>;
#[doc = "Field `SBERRADDR0` writer - 10:0\\]
Single Bit ECC Error Address This register holds the address when a single bit error is generated from SECDED block while reading the MibSPI (Transmit) TXRAM. The TXRAM can be read either by CPU or by the MibSPI Sequencer logic for transmission. This error address is frozen from being updated until it is read by the VBUSP host. Reading this register clears its contents to the default value of 0x000. Writes to this register are ignored."]
pub type Sberraddr0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `NU2` reader - 31:11\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu2R = crate::FieldReader<u32>;
#[doc = "Field `NU2` writer - 31:11\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Single Bit ECC Error Address This register holds the address when a single bit error is generated from SECDED block while reading the MibSPI (Transmit) TXRAM. The TXRAM can be read either by CPU or by the MibSPI Sequencer logic for transmission. This error address is frozen from being updated until it is read by the VBUSP host. Reading this register clears its contents to the default value of 0x000. Writes to this register are ignored."]
    #[inline(always)]
    pub fn sberraddr0(&self) -> Sberraddr0R {
        Sberraddr0R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Single Bit ECC Error Address This register holds the address when a single bit error is generated from SECDED block while reading the MibSPI (Transmit) TXRAM. The TXRAM can be read either by CPU or by the MibSPI Sequencer logic for transmission. This error address is frozen from being updated until it is read by the VBUSP host. Reading this register clears its contents to the default value of 0x000. Writes to this register are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn sberraddr0(&mut self) -> Sberraddr0W<Sberraddr0Spec> {
        Sberraddr0W::new(self, 0)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<Sberraddr0Spec> {
        Nu2W::new(self, 11)
    }
}
#[doc = "Single Bit ECC Error Address Register - TXRAM\n\nYou can [`read`](crate::Reg::read) this register and get [`sberraddr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sberraddr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sberraddr0Spec;
impl crate::RegisterSpec for Sberraddr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sberraddr0::R`](R) reader structure"]
impl crate::Readable for Sberraddr0Spec {}
#[doc = "`write(|w| ..)` method takes [`sberraddr0::W`](W) writer structure"]
impl crate::Writable for Sberraddr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SBERRADDR0 to value 0"]
impl crate::Resettable for Sberraddr0Spec {
    const RESET_VALUE: u32 = 0;
}
