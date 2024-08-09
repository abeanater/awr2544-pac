#[doc = "Register `TCM_PARITY_CTRL` reader"]
pub type R = crate::R<TcmParityCtrlSpec>;
#[doc = "Register `TCM_PARITY_CTRL` writer"]
pub type W = crate::W<TcmParityCtrlSpec>;
#[doc = "Field `atcm0_erraddr_clr` reader - 2:0\\]
Pulse bit-field writing 3'b111 clears the Address latched after parity error for ATCM of CR5A"]
pub type Atcm0ErraddrClrR = crate::FieldReader;
#[doc = "Field `atcm0_erraddr_clr` writer - 2:0\\]
Pulse bit-field writing 3'b111 clears the Address latched after parity error for ATCM of CR5A"]
pub type Atcm0ErraddrClrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `atcm1_erraddr_clr` reader - 6:4\\]
RESERVED: Dont Use"]
pub type Atcm1ErraddrClrR = crate::FieldReader;
#[doc = "Field `atcm1_erraddr_clr` writer - 6:4\\]
RESERVED: Dont Use"]
pub type Atcm1ErraddrClrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `b0tcm0_erraddr_clr` reader - 10:8\\]
RESERVED: Dont Use"]
pub type B0tcm0ErraddrClrR = crate::FieldReader;
#[doc = "Field `b0tcm0_erraddr_clr` writer - 10:8\\]
RESERVED: Dont Use"]
pub type B0tcm0ErraddrClrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `b0cm1_erraddr_clr` reader - 14:12\\]
RESERVED: Dont Use"]
pub type B0cm1ErraddrClrR = crate::FieldReader;
#[doc = "Field `b0cm1_erraddr_clr` writer - 14:12\\]
RESERVED: Dont Use"]
pub type B0cm1ErraddrClrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `b1tcm0_erraddr_clr` reader - 18:16\\]
Write pulse bit field: writing 3'b111 clears the Address latched after parity error for B1TCM of CR5A"]
pub type B1tcm0ErraddrClrR = crate::FieldReader;
#[doc = "Field `b1tcm0_erraddr_clr` writer - 18:16\\]
Write pulse bit field: writing 3'b111 clears the Address latched after parity error for B1TCM of CR5A"]
pub type B1tcm0ErraddrClrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `b1tcm1_erraddr_clr` reader - 22:20\\]
RESERVED: Dont Use"]
pub type B1tcm1ErraddrClrR = crate::FieldReader;
#[doc = "Field `b1tcm1_erraddr_clr` writer - 22:20\\]
RESERVED: Dont Use"]
pub type B1tcm1ErraddrClrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Pulse bit-field writing 3'b111 clears the Address latched after parity error for ATCM of CR5A"]
    #[inline(always)]
    pub fn atcm0_erraddr_clr(&self) -> Atcm0ErraddrClrR {
        Atcm0ErraddrClrR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn atcm1_erraddr_clr(&self) -> Atcm1ErraddrClrR {
        Atcm1ErraddrClrR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn b0tcm0_erraddr_clr(&self) -> B0tcm0ErraddrClrR {
        B0tcm0ErraddrClrR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn b0cm1_erraddr_clr(&self) -> B0cm1ErraddrClrR {
        B0cm1ErraddrClrR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Write pulse bit field: writing 3'b111 clears the Address latched after parity error for B1TCM of CR5A"]
    #[inline(always)]
    pub fn b1tcm0_erraddr_clr(&self) -> B1tcm0ErraddrClrR {
        B1tcm0ErraddrClrR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - 22:20\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn b1tcm1_erraddr_clr(&self) -> B1tcm1ErraddrClrR {
        B1tcm1ErraddrClrR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Pulse bit-field writing 3'b111 clears the Address latched after parity error for ATCM of CR5A"]
    #[inline(always)]
    #[must_use]
    pub fn atcm0_erraddr_clr(&mut self) -> Atcm0ErraddrClrW<TcmParityCtrlSpec> {
        Atcm0ErraddrClrW::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn atcm1_erraddr_clr(&mut self) -> Atcm1ErraddrClrW<TcmParityCtrlSpec> {
        Atcm1ErraddrClrW::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn b0tcm0_erraddr_clr(&mut self) -> B0tcm0ErraddrClrW<TcmParityCtrlSpec> {
        B0tcm0ErraddrClrW::new(self, 8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn b0cm1_erraddr_clr(&mut self) -> B0cm1ErraddrClrW<TcmParityCtrlSpec> {
        B0cm1ErraddrClrW::new(self, 12)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Write pulse bit field: writing 3'b111 clears the Address latched after parity error for B1TCM of CR5A"]
    #[inline(always)]
    #[must_use]
    pub fn b1tcm0_erraddr_clr(&mut self) -> B1tcm0ErraddrClrW<TcmParityCtrlSpec> {
        B1tcm0ErraddrClrW::new(self, 16)
    }
    #[doc = "Bits 20:22 - 22:20\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn b1tcm1_erraddr_clr(&mut self) -> B1tcm1ErraddrClrW<TcmParityCtrlSpec> {
        B1tcm1ErraddrClrW::new(self, 20)
    }
}
#[doc = "TCM_PARITY_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_parity_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_parity_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcmParityCtrlSpec;
impl crate::RegisterSpec for TcmParityCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcm_parity_ctrl::R`](R) reader structure"]
impl crate::Readable for TcmParityCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tcm_parity_ctrl::W`](W) writer structure"]
impl crate::Writable for TcmParityCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCM_PARITY_CTRL to value 0"]
impl crate::Resettable for TcmParityCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
