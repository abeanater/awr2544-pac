#[doc = "Register `TCM_PARITY_ERRFRC` reader"]
pub type R = crate::R<TcmParityErrfrcSpec>;
#[doc = "Register `TCM_PARITY_ERRFRC` writer"]
pub type W = crate::W<TcmParityErrfrcSpec>;
#[doc = "Field `atcm0` reader - 2:0\\]
Write pulse bit field: writing 3'b111 forces a parity error for ATCM of CR5A"]
pub type Atcm0R = crate::FieldReader;
#[doc = "Field `atcm0` writer - 2:0\\]
Write pulse bit field: writing 3'b111 forces a parity error for ATCM of CR5A"]
pub type Atcm0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `atcm1` reader - 6:4\\]
RESERVED: Dont Use"]
pub type Atcm1R = crate::FieldReader;
#[doc = "Field `atcm1` writer - 6:4\\]
RESERVED: Dont Use"]
pub type Atcm1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `b0tcm0` reader - 10:8\\]
Write pulse bit field: writing 3'b111 forces a parity error for B0TCM of CR5A"]
pub type B0tcm0R = crate::FieldReader;
#[doc = "Field `b0tcm0` writer - 10:8\\]
Write pulse bit field: writing 3'b111 forces a parity error for B0TCM of CR5A"]
pub type B0tcm0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `b0tcm1` reader - 14:12\\]
RESERVED: Dont Use"]
pub type B0tcm1R = crate::FieldReader;
#[doc = "Field `b0tcm1` writer - 14:12\\]
RESERVED: Dont Use"]
pub type B0tcm1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `b1tcm0` reader - 18:16\\]
Write pulse bit field: writing 3'b111 forces a parity error for B1TCM of CR5A"]
pub type B1tcm0R = crate::FieldReader;
#[doc = "Field `b1tcm0` writer - 18:16\\]
Write pulse bit field: writing 3'b111 forces a parity error for B1TCM of CR5A"]
pub type B1tcm0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `b1tcm1` reader - 22:20\\]
RESERVED: Dont Use"]
pub type B1tcm1R = crate::FieldReader;
#[doc = "Field `b1tcm1` writer - 22:20\\]
RESERVED: Dont Use"]
pub type B1tcm1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Write pulse bit field: writing 3'b111 forces a parity error for ATCM of CR5A"]
    #[inline(always)]
    pub fn atcm0(&self) -> Atcm0R {
        Atcm0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn atcm1(&self) -> Atcm1R {
        Atcm1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Write pulse bit field: writing 3'b111 forces a parity error for B0TCM of CR5A"]
    #[inline(always)]
    pub fn b0tcm0(&self) -> B0tcm0R {
        B0tcm0R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn b0tcm1(&self) -> B0tcm1R {
        B0tcm1R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Write pulse bit field: writing 3'b111 forces a parity error for B1TCM of CR5A"]
    #[inline(always)]
    pub fn b1tcm0(&self) -> B1tcm0R {
        B1tcm0R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - 22:20\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn b1tcm1(&self) -> B1tcm1R {
        B1tcm1R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Write pulse bit field: writing 3'b111 forces a parity error for ATCM of CR5A"]
    #[inline(always)]
    #[must_use]
    pub fn atcm0(&mut self) -> Atcm0W<TcmParityErrfrcSpec> {
        Atcm0W::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn atcm1(&mut self) -> Atcm1W<TcmParityErrfrcSpec> {
        Atcm1W::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Write pulse bit field: writing 3'b111 forces a parity error for B0TCM of CR5A"]
    #[inline(always)]
    #[must_use]
    pub fn b0tcm0(&mut self) -> B0tcm0W<TcmParityErrfrcSpec> {
        B0tcm0W::new(self, 8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn b0tcm1(&mut self) -> B0tcm1W<TcmParityErrfrcSpec> {
        B0tcm1W::new(self, 12)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Write pulse bit field: writing 3'b111 forces a parity error for B1TCM of CR5A"]
    #[inline(always)]
    #[must_use]
    pub fn b1tcm0(&mut self) -> B1tcm0W<TcmParityErrfrcSpec> {
        B1tcm0W::new(self, 16)
    }
    #[doc = "Bits 20:22 - 22:20\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn b1tcm1(&mut self) -> B1tcm1W<TcmParityErrfrcSpec> {
        B1tcm1W::new(self, 20)
    }
}
#[doc = "TCM_PARITY_ERRFRC\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_parity_errfrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_parity_errfrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcmParityErrfrcSpec;
impl crate::RegisterSpec for TcmParityErrfrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcm_parity_errfrc::R`](R) reader structure"]
impl crate::Readable for TcmParityErrfrcSpec {}
#[doc = "`write(|w| ..)` method takes [`tcm_parity_errfrc::W`](W) writer structure"]
impl crate::Writable for TcmParityErrfrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCM_PARITY_ERRFRC to value 0"]
impl crate::Resettable for TcmParityErrfrcSpec {
    const RESET_VALUE: u32 = 0;
}
