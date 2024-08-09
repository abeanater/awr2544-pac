#[doc = "Register `STCSCSCR` reader"]
pub type R = crate::R<StcscscrSpec>;
#[doc = "Register `STCSCSCR` writer"]
pub type W = crate::W<StcscscrSpec>;
#[doc = "Field `SELF_CHECK_KEY_B4` reader - 3:0\\]
Signature compare logic self check key enable/disable (RWP - Read, Priviledge Mode Write only) 1010 = Signature compare logic Self Check is enabled All values other than 1010 = Signature compare logic Self Check is disabled"]
pub type SelfCheckKeyB4R = crate::FieldReader;
#[doc = "Field `SELF_CHECK_KEY_B4` writer - 3:0\\]
Signature compare logic self check key enable/disable (RWP - Read, Priviledge Mode Write only) 1010 = Signature compare logic Self Check is enabled All values other than 1010 = Signature compare logic Self Check is disabled"]
pub type SelfCheckKeyB4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FAULT_INS_B1` reader - 4:4\\]
Fault Insertion bit (RWP - Read, Priviledge Mode Write only) 0 = No fault insertion. 1 = Inserts fault in the logic unedr test which will make signature compare fail. This feature is used as diagnostic check of the STC IP."]
pub type FaultInsB1R = crate::BitReader;
#[doc = "Field `FAULT_INS_B1` writer - 4:4\\]
Fault Insertion bit (RWP - Read, Priviledge Mode Write only) 0 = No fault insertion. 1 = Inserts fault in the logic unedr test which will make signature compare fail. This feature is used as diagnostic check of the STC IP."]
pub type FaultInsB1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU7` reader - 31:5\\]
Reserved bits"]
pub type Nu7R = crate::FieldReader<u32>;
#[doc = "Field `NU7` writer - 31:5\\]
Reserved bits"]
pub type Nu7W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Signature compare logic self check key enable/disable (RWP - Read, Priviledge Mode Write only) 1010 = Signature compare logic Self Check is enabled All values other than 1010 = Signature compare logic Self Check is disabled"]
    #[inline(always)]
    pub fn self_check_key_b4(&self) -> SelfCheckKeyB4R {
        SelfCheckKeyB4R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Fault Insertion bit (RWP - Read, Priviledge Mode Write only) 0 = No fault insertion. 1 = Inserts fault in the logic unedr test which will make signature compare fail. This feature is used as diagnostic check of the STC IP."]
    #[inline(always)]
    pub fn fault_ins_b1(&self) -> FaultInsB1R {
        FaultInsB1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Reserved bits"]
    #[inline(always)]
    pub fn nu7(&self) -> Nu7R {
        Nu7R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Signature compare logic self check key enable/disable (RWP - Read, Priviledge Mode Write only) 1010 = Signature compare logic Self Check is enabled All values other than 1010 = Signature compare logic Self Check is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn self_check_key_b4(&mut self) -> SelfCheckKeyB4W<StcscscrSpec> {
        SelfCheckKeyB4W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Fault Insertion bit (RWP - Read, Priviledge Mode Write only) 0 = No fault insertion. 1 = Inserts fault in the logic unedr test which will make signature compare fail. This feature is used as diagnostic check of the STC IP."]
    #[inline(always)]
    #[must_use]
    pub fn fault_ins_b1(&mut self) -> FaultInsB1W<StcscscrSpec> {
        FaultInsB1W::new(self, 4)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Reserved bits"]
    #[inline(always)]
    #[must_use]
    pub fn nu7(&mut self) -> Nu7W<StcscscrSpec> {
        Nu7W::new(self, 5)
    }
}
#[doc = "Signature compare Self Check Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stcscscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcscscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StcscscrSpec;
impl crate::RegisterSpec for StcscscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stcscscr::R`](R) reader structure"]
impl crate::Readable for StcscscrSpec {}
#[doc = "`write(|w| ..)` method takes [`stcscscr::W`](W) writer structure"]
impl crate::Writable for StcscscrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STCSCSCR to value 0"]
impl crate::Resettable for StcscscrSpec {
    const RESET_VALUE: u32 = 0;
}
