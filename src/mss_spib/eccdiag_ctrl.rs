#[doc = "Register `ECCDIAG_CTRL` reader"]
pub type R = crate::R<EccdiagCtrlSpec>;
#[doc = "Register `ECCDIAG_CTRL` writer"]
pub type W = crate::W<EccdiagCtrlSpec>;
#[doc = "Field `ECCDIAG_EN` reader - 3:0\\]
ECC Diagnostic mode Enable Key bits. 0101 : Diagnostic mode is enabled. Writes and reads from ECC bits allowed from the ECC address space. Refer to Section 9 for details on ECC/Parity address space. Others : Diagnostic mode is disabled. No writes to ECC bits are ignored, reads return ΓÇÿ0ΓÇÖ."]
pub type EccdiagEnR = crate::FieldReader;
#[doc = "Field `ECCDIAG_EN` writer - 3:0\\]
ECC Diagnostic mode Enable Key bits. 0101 : Diagnostic mode is enabled. Writes and reads from ECC bits allowed from the ECC address space. Refer to Section 9 for details on ECC/Parity address space. Others : Diagnostic mode is disabled. No writes to ECC bits are ignored, reads return ΓÇÿ0ΓÇÖ."]
pub type EccdiagEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU` reader - 31:4\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:4\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
ECC Diagnostic mode Enable Key bits. 0101 : Diagnostic mode is enabled. Writes and reads from ECC bits allowed from the ECC address space. Refer to Section 9 for details on ECC/Parity address space. Others : Diagnostic mode is disabled. No writes to ECC bits are ignored, reads return ΓÇÿ0ΓÇÖ."]
    #[inline(always)]
    pub fn eccdiag_en(&self) -> EccdiagEnR {
        EccdiagEnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
ECC Diagnostic mode Enable Key bits. 0101 : Diagnostic mode is enabled. Writes and reads from ECC bits allowed from the ECC address space. Refer to Section 9 for details on ECC/Parity address space. Others : Diagnostic mode is disabled. No writes to ECC bits are ignored, reads return ΓÇÿ0ΓÇÖ."]
    #[inline(always)]
    #[must_use]
    pub fn eccdiag_en(&mut self) -> EccdiagEnW<EccdiagCtrlSpec> {
        EccdiagEnW::new(self, 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<EccdiagCtrlSpec> {
        NuW::new(self, 4)
    }
}
#[doc = "ECC Diagnostic Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccdiag_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccdiag_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccdiagCtrlSpec;
impl crate::RegisterSpec for EccdiagCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccdiag_ctrl::R`](R) reader structure"]
impl crate::Readable for EccdiagCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`eccdiag_ctrl::W`](W) writer structure"]
impl crate::Writable for EccdiagCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCDIAG_CTRL to value 0"]
impl crate::Resettable for EccdiagCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
