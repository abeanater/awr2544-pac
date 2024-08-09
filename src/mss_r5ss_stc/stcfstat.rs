#[doc = "Register `STCFSTAT` reader"]
pub type R = crate::R<StcfstatSpec>;
#[doc = "Register `STCFSTAT` writer"]
pub type W = crate::W<StcfstatSpec>;
#[doc = "Field `CPU1_FAIL_B1` reader - 0:0\\]
Tells whether MISR mismatch happenned in CORE1 (RCP - Read, Clear on Writing in Priviledge Mode) Applicable to all segments. 0 = No MISR mismatch for CORE1 1 = Self test run failed due to MISR mismatch for CORE1"]
pub type Cpu1FailB1R = crate::BitReader;
#[doc = "Field `CPU1_FAIL_B1` writer - 0:0\\]
Tells whether MISR mismatch happenned in CORE1 (RCP - Read, Clear on Writing in Priviledge Mode) Applicable to all segments. 0 = No MISR mismatch for CORE1 1 = Self test run failed due to MISR mismatch for CORE1"]
pub type Cpu1FailB1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU2_FAIL_B1` reader - 1:1\\]
Tells whether MISR mismatch happenned in CORE2 when in Segment0 mode (RCP - Read, Clear on Writing in Priviledge Mode) 0 = No MISR mismatch for CORE2 1 = Self test run failed due to MISR mismatch for CORE2"]
pub type Cpu2FailB1R = crate::BitReader;
#[doc = "Field `CPU2_FAIL_B1` writer - 1:1\\]
Tells whether MISR mismatch happenned in CORE2 when in Segment0 mode (RCP - Read, Clear on Writing in Priviledge Mode) 0 = No MISR mismatch for CORE2 1 = Self test run failed due to MISR mismatch for CORE2"]
pub type Cpu2FailB1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TO_ER_B1` reader - 2:2\\]
Tells whether self test failed because of time out error (RCP - Read, Clear on Writing in Priviledge Mode) 0 = No time out error occurred 1 = SelfTest run failed due to a timeout error"]
pub type ToErB1R = crate::BitReader;
#[doc = "Field `TO_ER_B1` writer - 2:2\\]
Tells whether self test failed because of time out error (RCP - Read, Clear on Writing in Priviledge Mode) 0 = No time out error occurred 1 = SelfTest run failed due to a timeout error"]
pub type ToErB1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSEG_ID` reader - 4:3\\]
Failed Segment ID (RCP - Read, Clear on Writing in Priviledge Mode) This field captures the Segment number for which any of the failures like TO_ER_B1, CPU1_FAIL_B1 and CPU2_FAIL_B1 occur. 00 = Failure on Segment 0 01 = Failure on Segment 1 10 = Failure on Segment 2 11 = Failure on Segment 3"]
pub type FsegIdR = crate::FieldReader;
#[doc = "Field `FSEG_ID` writer - 4:3\\]
Failed Segment ID (RCP - Read, Clear on Writing in Priviledge Mode) This field captures the Segment number for which any of the failures like TO_ER_B1, CPU1_FAIL_B1 and CPU2_FAIL_B1 occur. 00 = Failure on Segment 0 01 = Failure on Segment 1 10 = Failure on Segment 2 11 = Failure on Segment 3"]
pub type FsegIdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NU6` reader - 31:5\\]
Reserved bits"]
pub type Nu6R = crate::FieldReader<u32>;
#[doc = "Field `NU6` writer - 31:5\\]
Reserved bits"]
pub type Nu6W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Tells whether MISR mismatch happenned in CORE1 (RCP - Read, Clear on Writing in Priviledge Mode) Applicable to all segments. 0 = No MISR mismatch for CORE1 1 = Self test run failed due to MISR mismatch for CORE1"]
    #[inline(always)]
    pub fn cpu1_fail_b1(&self) -> Cpu1FailB1R {
        Cpu1FailB1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Tells whether MISR mismatch happenned in CORE2 when in Segment0 mode (RCP - Read, Clear on Writing in Priviledge Mode) 0 = No MISR mismatch for CORE2 1 = Self test run failed due to MISR mismatch for CORE2"]
    #[inline(always)]
    pub fn cpu2_fail_b1(&self) -> Cpu2FailB1R {
        Cpu2FailB1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Tells whether self test failed because of time out error (RCP - Read, Clear on Writing in Priviledge Mode) 0 = No time out error occurred 1 = SelfTest run failed due to a timeout error"]
    #[inline(always)]
    pub fn to_er_b1(&self) -> ToErB1R {
        ToErB1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Failed Segment ID (RCP - Read, Clear on Writing in Priviledge Mode) This field captures the Segment number for which any of the failures like TO_ER_B1, CPU1_FAIL_B1 and CPU2_FAIL_B1 occur. 00 = Failure on Segment 0 01 = Failure on Segment 1 10 = Failure on Segment 2 11 = Failure on Segment 3"]
    #[inline(always)]
    pub fn fseg_id(&self) -> FsegIdR {
        FsegIdR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Reserved bits"]
    #[inline(always)]
    pub fn nu6(&self) -> Nu6R {
        Nu6R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Tells whether MISR mismatch happenned in CORE1 (RCP - Read, Clear on Writing in Priviledge Mode) Applicable to all segments. 0 = No MISR mismatch for CORE1 1 = Self test run failed due to MISR mismatch for CORE1"]
    #[inline(always)]
    #[must_use]
    pub fn cpu1_fail_b1(&mut self) -> Cpu1FailB1W<StcfstatSpec> {
        Cpu1FailB1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Tells whether MISR mismatch happenned in CORE2 when in Segment0 mode (RCP - Read, Clear on Writing in Priviledge Mode) 0 = No MISR mismatch for CORE2 1 = Self test run failed due to MISR mismatch for CORE2"]
    #[inline(always)]
    #[must_use]
    pub fn cpu2_fail_b1(&mut self) -> Cpu2FailB1W<StcfstatSpec> {
        Cpu2FailB1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Tells whether self test failed because of time out error (RCP - Read, Clear on Writing in Priviledge Mode) 0 = No time out error occurred 1 = SelfTest run failed due to a timeout error"]
    #[inline(always)]
    #[must_use]
    pub fn to_er_b1(&mut self) -> ToErB1W<StcfstatSpec> {
        ToErB1W::new(self, 2)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Failed Segment ID (RCP - Read, Clear on Writing in Priviledge Mode) This field captures the Segment number for which any of the failures like TO_ER_B1, CPU1_FAIL_B1 and CPU2_FAIL_B1 occur. 00 = Failure on Segment 0 01 = Failure on Segment 1 10 = Failure on Segment 2 11 = Failure on Segment 3"]
    #[inline(always)]
    #[must_use]
    pub fn fseg_id(&mut self) -> FsegIdW<StcfstatSpec> {
        FsegIdW::new(self, 3)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Reserved bits"]
    #[inline(always)]
    #[must_use]
    pub fn nu6(&mut self) -> Nu6W<StcfstatSpec> {
        Nu6W::new(self, 5)
    }
}
#[doc = "Fail Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stcfstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcfstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StcfstatSpec;
impl crate::RegisterSpec for StcfstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stcfstat::R`](R) reader structure"]
impl crate::Readable for StcfstatSpec {}
#[doc = "`write(|w| ..)` method takes [`stcfstat::W`](W) writer structure"]
impl crate::Writable for StcfstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STCFSTAT to value 0"]
impl crate::Resettable for StcfstatSpec {
    const RESET_VALUE: u32 = 0;
}
