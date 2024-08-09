#[doc = "Register `STCGSTAT` reader"]
pub type R = crate::R<StcgstatSpec>;
#[doc = "Register `STCGSTAT` writer"]
pub type W = crate::W<StcgstatSpec>;
#[doc = "Field `TEST_DONE` reader - 0:0\\]
Test_done_flag (RCP - Read, Clear on Writing in Priviledge Mode) 0 = Not completed 1 = SelfTest run Completed"]
pub type TestDoneR = crate::BitReader;
#[doc = "Field `TEST_DONE` writer - 0:0\\]
Test_done_flag (RCP - Read, Clear on Writing in Priviledge Mode) 0 = Not completed 1 = SelfTest run Completed"]
pub type TestDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEST_FAIL` reader - 1:1\\]
Test_fail flag (RCP - Read, Clear on Writing in Priviledge Mode) 0 = Self test run has not failed 1 = SelfTest run has failed. Write Clear."]
pub type TestFailR = crate::BitReader;
#[doc = "Field `TEST_FAIL` writer - 1:1\\]
Test_fail flag (RCP - Read, Clear on Writing in Priviledge Mode) 0 = Self test run has not failed 1 = SelfTest run has failed. Write Clear."]
pub type TestFailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU5` reader - 7:2\\]
Reserved bits"]
pub type Nu5R = crate::FieldReader;
#[doc = "Field `NU5` writer - 7:2\\]
Reserved bits"]
pub type Nu5W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ST_ACTIVE` reader - 11:8\\]
Tells whether self test is currently active or not. 1010 = Self test is active Others = SelfTest is not active Once the self-test completes and ST_ENA_B4 key is cleared, this field will reflect the inactive value."]
pub type StActiveR = crate::FieldReader;
#[doc = "Field `ST_ACTIVE` writer - 11:8\\]
Tells whether self test is currently active or not. 1010 = Self test is active Others = SelfTest is not active Once the self-test completes and ST_ENA_B4 key is cleared, this field will reflect the inactive value."]
pub type StActiveW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU4` reader - 31:12\\]
Reserved bits"]
pub type Nu4R = crate::FieldReader<u32>;
#[doc = "Field `NU4` writer - 31:12\\]
Reserved bits"]
pub type Nu4W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Test_done_flag (RCP - Read, Clear on Writing in Priviledge Mode) 0 = Not completed 1 = SelfTest run Completed"]
    #[inline(always)]
    pub fn test_done(&self) -> TestDoneR {
        TestDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Test_fail flag (RCP - Read, Clear on Writing in Priviledge Mode) 0 = Self test run has not failed 1 = SelfTest run has failed. Write Clear."]
    #[inline(always)]
    pub fn test_fail(&self) -> TestFailR {
        TestFailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Reserved bits"]
    #[inline(always)]
    pub fn nu5(&self) -> Nu5R {
        Nu5R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Tells whether self test is currently active or not. 1010 = Self test is active Others = SelfTest is not active Once the self-test completes and ST_ENA_B4 key is cleared, this field will reflect the inactive value."]
    #[inline(always)]
    pub fn st_active(&self) -> StActiveR {
        StActiveR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Reserved bits"]
    #[inline(always)]
    pub fn nu4(&self) -> Nu4R {
        Nu4R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Test_done_flag (RCP - Read, Clear on Writing in Priviledge Mode) 0 = Not completed 1 = SelfTest run Completed"]
    #[inline(always)]
    #[must_use]
    pub fn test_done(&mut self) -> TestDoneW<StcgstatSpec> {
        TestDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Test_fail flag (RCP - Read, Clear on Writing in Priviledge Mode) 0 = Self test run has not failed 1 = SelfTest run has failed. Write Clear."]
    #[inline(always)]
    #[must_use]
    pub fn test_fail(&mut self) -> TestFailW<StcgstatSpec> {
        TestFailW::new(self, 1)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Reserved bits"]
    #[inline(always)]
    #[must_use]
    pub fn nu5(&mut self) -> Nu5W<StcgstatSpec> {
        Nu5W::new(self, 2)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Tells whether self test is currently active or not. 1010 = Self test is active Others = SelfTest is not active Once the self-test completes and ST_ENA_B4 key is cleared, this field will reflect the inactive value."]
    #[inline(always)]
    #[must_use]
    pub fn st_active(&mut self) -> StActiveW<StcgstatSpec> {
        StActiveW::new(self, 8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Reserved bits"]
    #[inline(always)]
    #[must_use]
    pub fn nu4(&mut self) -> Nu4W<StcgstatSpec> {
        Nu4W::new(self, 12)
    }
}
#[doc = "Global Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stcgstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcgstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StcgstatSpec;
impl crate::RegisterSpec for StcgstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stcgstat::R`](R) reader structure"]
impl crate::Readable for StcgstatSpec {}
#[doc = "`write(|w| ..)` method takes [`stcgstat::W`](W) writer structure"]
impl crate::Writable for StcgstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STCGSTAT to value 0"]
impl crate::Resettable for StcgstatSpec {
    const RESET_VALUE: u32 = 0;
}
