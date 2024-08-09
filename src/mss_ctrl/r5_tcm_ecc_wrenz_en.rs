#[doc = "Register `R5_TCM_ECC_WRENZ_EN` reader"]
pub type R = crate::R<R5TcmEccWrenzEnSpec>;
#[doc = "Register `R5_TCM_ECC_WRENZ_EN` writer"]
pub type W = crate::W<R5TcmEccWrenzEnSpec>;
#[doc = "Field `cpu0_tcma_wrenz_en` reader - 2:0\\]
writing '000' blocks the writes to ECC-bits of TCMA-RAM of CR5A. Writing '111' unblocks the writes to ECC-bits of TCMA-RAM of CR5A"]
pub type Cpu0TcmaWrenzEnR = crate::FieldReader;
#[doc = "Field `cpu0_tcma_wrenz_en` writer - 2:0\\]
writing '000' blocks the writes to ECC-bits of TCMA-RAM of CR5A. Writing '111' unblocks the writes to ECC-bits of TCMA-RAM of CR5A"]
pub type Cpu0TcmaWrenzEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cpu0_tcmb0_wrenz_en` reader - 6:4\\]
writing '000' blocks the writes to ECC-bits of TCMB0-RAM of CR5A. Writing '111' unblocks the writes to ECC-bits of TCMB0-RAM of CR5A"]
pub type Cpu0Tcmb0WrenzEnR = crate::FieldReader;
#[doc = "Field `cpu0_tcmb0_wrenz_en` writer - 6:4\\]
writing '000' blocks the writes to ECC-bits of TCMB0-RAM of CR5A. Writing '111' unblocks the writes to ECC-bits of TCMB0-RAM of CR5A"]
pub type Cpu0Tcmb0WrenzEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cpu0_tcmb1_wrenz_en` reader - 10:8\\]
writing '000' blocks the writes to ECC-bits of TCMB0-RAM of CR5A. Writing '111' unblocks the writes to ECC-bits of TCMB1-RAM of CR5A"]
pub type Cpu0Tcmb1WrenzEnR = crate::FieldReader;
#[doc = "Field `cpu0_tcmb1_wrenz_en` writer - 10:8\\]
writing '000' blocks the writes to ECC-bits of TCMB0-RAM of CR5A. Writing '111' unblocks the writes to ECC-bits of TCMB1-RAM of CR5A"]
pub type Cpu0Tcmb1WrenzEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cpu1_tcma_wrenz_en` reader - 14:12\\]
RESERVED: Dont Use"]
pub type Cpu1TcmaWrenzEnR = crate::FieldReader;
#[doc = "Field `cpu1_tcma_wrenz_en` writer - 14:12\\]
RESERVED: Dont Use"]
pub type Cpu1TcmaWrenzEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cpu1_tcmb0_wrenz_en` reader - 18:16\\]
RESERVED: Dont Use"]
pub type Cpu1Tcmb0WrenzEnR = crate::FieldReader;
#[doc = "Field `cpu1_tcmb0_wrenz_en` writer - 18:16\\]
RESERVED: Dont Use"]
pub type Cpu1Tcmb0WrenzEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cpu1_tcmb1_wrenz_en` reader - 22:20\\]
RESERVED: Dont Use"]
pub type Cpu1Tcmb1WrenzEnR = crate::FieldReader;
#[doc = "Field `cpu1_tcmb1_wrenz_en` writer - 22:20\\]
RESERVED: Dont Use"]
pub type Cpu1Tcmb1WrenzEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
writing '000' blocks the writes to ECC-bits of TCMA-RAM of CR5A. Writing '111' unblocks the writes to ECC-bits of TCMA-RAM of CR5A"]
    #[inline(always)]
    pub fn cpu0_tcma_wrenz_en(&self) -> Cpu0TcmaWrenzEnR {
        Cpu0TcmaWrenzEnR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
writing '000' blocks the writes to ECC-bits of TCMB0-RAM of CR5A. Writing '111' unblocks the writes to ECC-bits of TCMB0-RAM of CR5A"]
    #[inline(always)]
    pub fn cpu0_tcmb0_wrenz_en(&self) -> Cpu0Tcmb0WrenzEnR {
        Cpu0Tcmb0WrenzEnR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
writing '000' blocks the writes to ECC-bits of TCMB0-RAM of CR5A. Writing '111' unblocks the writes to ECC-bits of TCMB1-RAM of CR5A"]
    #[inline(always)]
    pub fn cpu0_tcmb1_wrenz_en(&self) -> Cpu0Tcmb1WrenzEnR {
        Cpu0Tcmb1WrenzEnR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn cpu1_tcma_wrenz_en(&self) -> Cpu1TcmaWrenzEnR {
        Cpu1TcmaWrenzEnR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn cpu1_tcmb0_wrenz_en(&self) -> Cpu1Tcmb0WrenzEnR {
        Cpu1Tcmb0WrenzEnR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - 22:20\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn cpu1_tcmb1_wrenz_en(&self) -> Cpu1Tcmb1WrenzEnR {
        Cpu1Tcmb1WrenzEnR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
writing '000' blocks the writes to ECC-bits of TCMA-RAM of CR5A. Writing '111' unblocks the writes to ECC-bits of TCMA-RAM of CR5A"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_tcma_wrenz_en(&mut self) -> Cpu0TcmaWrenzEnW<R5TcmEccWrenzEnSpec> {
        Cpu0TcmaWrenzEnW::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
writing '000' blocks the writes to ECC-bits of TCMB0-RAM of CR5A. Writing '111' unblocks the writes to ECC-bits of TCMB0-RAM of CR5A"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_tcmb0_wrenz_en(&mut self) -> Cpu0Tcmb0WrenzEnW<R5TcmEccWrenzEnSpec> {
        Cpu0Tcmb0WrenzEnW::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
writing '000' blocks the writes to ECC-bits of TCMB0-RAM of CR5A. Writing '111' unblocks the writes to ECC-bits of TCMB1-RAM of CR5A"]
    #[inline(always)]
    #[must_use]
    pub fn cpu0_tcmb1_wrenz_en(&mut self) -> Cpu0Tcmb1WrenzEnW<R5TcmEccWrenzEnSpec> {
        Cpu0Tcmb1WrenzEnW::new(self, 8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn cpu1_tcma_wrenz_en(&mut self) -> Cpu1TcmaWrenzEnW<R5TcmEccWrenzEnSpec> {
        Cpu1TcmaWrenzEnW::new(self, 12)
    }
    #[doc = "Bits 16:18 - 18:16\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn cpu1_tcmb0_wrenz_en(&mut self) -> Cpu1Tcmb0WrenzEnW<R5TcmEccWrenzEnSpec> {
        Cpu1Tcmb0WrenzEnW::new(self, 16)
    }
    #[doc = "Bits 20:22 - 22:20\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn cpu1_tcmb1_wrenz_en(&mut self) -> Cpu1Tcmb1WrenzEnW<R5TcmEccWrenzEnSpec> {
        Cpu1Tcmb1WrenzEnW::new(self, 20)
    }
}
#[doc = "R5_TCM_ECC_WRENZ_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`r5_tcm_ecc_wrenz_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5_tcm_ecc_wrenz_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R5TcmEccWrenzEnSpec;
impl crate::RegisterSpec for R5TcmEccWrenzEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r5_tcm_ecc_wrenz_en::R`](R) reader structure"]
impl crate::Readable for R5TcmEccWrenzEnSpec {}
#[doc = "`write(|w| ..)` method takes [`r5_tcm_ecc_wrenz_en::W`](W) writer structure"]
impl crate::Writable for R5TcmEccWrenzEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R5_TCM_ECC_WRENZ_EN to value 0"]
impl crate::Resettable for R5TcmEccWrenzEnSpec {
    const RESET_VALUE: u32 = 0;
}
