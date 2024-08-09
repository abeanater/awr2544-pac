#[doc = "Register `err_stat1` reader"]
pub type R = crate::R<ErrStat1Spec>;
#[doc = "Register `err_stat1` writer"]
pub type W = crate::W<ErrStat1Spec>;
#[doc = "Field `LEVEL_SINGLE_BIT` reader - 1:0\\]
Level Single Bit Error Status"]
pub type LevelSingleBitR = crate::FieldReader;
#[doc = "Field `LEVEL_SINGLE_BIT` writer - 1:0\\]
Level Single Bit Error Status"]
pub type LevelSingleBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LEVEL_DOUBLE_BIT` reader - 3:2\\]
Level Double Bit Error Status"]
pub type LevelDoubleBitR = crate::FieldReader;
#[doc = "Field `LEVEL_DOUBLE_BIT` writer - 3:2\\]
Level Double Bit Error Status"]
pub type LevelDoubleBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SUCCESSIVE_SINGLEBIT_ERRORS` reader - 4:4\\]
successive single-bit errors have occurred while a writeback is still pending, Level interrupt"]
pub type SuccessiveSinglebitErrorsR = crate::BitReader;
#[doc = "Field `SUCCESSIVE_SINGLEBIT_ERRORS` writer - 4:4\\]
successive single-bit errors have occurred while a writeback is still pending, Level interrupt"]
pub type SuccessiveSinglebitErrorsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEVEL_PARITY_ERROR` reader - 6:5\\]
Level parity error Error Status"]
pub type LevelParityErrorR = crate::FieldReader;
#[doc = "Field `LEVEL_PARITY_ERROR` writer - 6:5\\]
Level parity error Error Status"]
pub type LevelParityErrorW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CONTROL_REGISTER_ERROR` reader - 7:7\\]
control register error pending, Level interrupt"]
pub type ControlRegisterErrorR = crate::BitReader;
#[doc = "Field `CONTROL_REGISTER_ERROR` writer - 7:7\\]
control register error pending, Level interrupt"]
pub type ControlRegisterErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_SINGLE_BIT` reader - 9:8\\]
Clear Single Bit Error Status"]
pub type ClearSingleBitR = crate::FieldReader;
#[doc = "Field `CLEAR_SINGLE_BIT` writer - 9:8\\]
Clear Single Bit Error Status"]
pub type ClearSingleBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLEAR_DOUBLE_BIT` reader - 11:10\\]
Clear Double Bit Error Status"]
pub type ClearDoubleBitR = crate::FieldReader;
#[doc = "Field `CLEAR_DOUBLE_BIT` writer - 11:10\\]
Clear Double Bit Error Status"]
pub type ClearDoubleBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLEAR_OTHER_ERROR` reader - 12:12\\]
Clear other Error Status"]
pub type ClearOtherErrorR = crate::BitReader;
#[doc = "Field `CLEAR_OTHER_ERROR` writer - 12:12\\]
Clear other Error Status"]
pub type ClearOtherErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_PARITY_ERROR` reader - 14:13\\]
Clear parity Error Status"]
pub type ClearParityErrorR = crate::FieldReader;
#[doc = "Field `CLEAR_PARITY_ERROR` writer - 14:13\\]
Clear parity Error Status"]
pub type ClearParityErrorW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLEAR_CONTROL_REG` reader - 15:15\\]
Clear control reg error Error Status, you must also re write the contorl ergister itself to clear this"]
pub type ClearControlRegR = crate::BitReader;
#[doc = "Field `CLEAR_CONTROL_REG` writer - 15:15\\]
Clear control reg error Error Status, you must also re write the contorl ergister itself to clear this"]
pub type ClearControlRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_BIT_THAT` reader - 31:16\\]
Data bit that corresponds to the single-bit error"]
pub type DataBitThatR = crate::FieldReader<u16>;
#[doc = "Field `DATA_BIT_THAT` writer - 31:16\\]
Data bit that corresponds to the single-bit error"]
pub type DataBitThatW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Level Single Bit Error Status"]
    #[inline(always)]
    pub fn level_single_bit(&self) -> LevelSingleBitR {
        LevelSingleBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Level Double Bit Error Status"]
    #[inline(always)]
    pub fn level_double_bit(&self) -> LevelDoubleBitR {
        LevelDoubleBitR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
successive single-bit errors have occurred while a writeback is still pending, Level interrupt"]
    #[inline(always)]
    pub fn successive_singlebit_errors(&self) -> SuccessiveSinglebitErrorsR {
        SuccessiveSinglebitErrorsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Level parity error Error Status"]
    #[inline(always)]
    pub fn level_parity_error(&self) -> LevelParityErrorR {
        LevelParityErrorR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
control register error pending, Level interrupt"]
    #[inline(always)]
    pub fn control_register_error(&self) -> ControlRegisterErrorR {
        ControlRegisterErrorR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Clear Single Bit Error Status"]
    #[inline(always)]
    pub fn clear_single_bit(&self) -> ClearSingleBitR {
        ClearSingleBitR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Clear Double Bit Error Status"]
    #[inline(always)]
    pub fn clear_double_bit(&self) -> ClearDoubleBitR {
        ClearDoubleBitR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Clear other Error Status"]
    #[inline(always)]
    pub fn clear_other_error(&self) -> ClearOtherErrorR {
        ClearOtherErrorR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Clear parity Error Status"]
    #[inline(always)]
    pub fn clear_parity_error(&self) -> ClearParityErrorR {
        ClearParityErrorR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Clear control reg error Error Status, you must also re write the contorl ergister itself to clear this"]
    #[inline(always)]
    pub fn clear_control_reg(&self) -> ClearControlRegR {
        ClearControlRegR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Data bit that corresponds to the single-bit error"]
    #[inline(always)]
    pub fn data_bit_that(&self) -> DataBitThatR {
        DataBitThatR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Level Single Bit Error Status"]
    #[inline(always)]
    #[must_use]
    pub fn level_single_bit(&mut self) -> LevelSingleBitW<ErrStat1Spec> {
        LevelSingleBitW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Level Double Bit Error Status"]
    #[inline(always)]
    #[must_use]
    pub fn level_double_bit(&mut self) -> LevelDoubleBitW<ErrStat1Spec> {
        LevelDoubleBitW::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
successive single-bit errors have occurred while a writeback is still pending, Level interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn successive_singlebit_errors(&mut self) -> SuccessiveSinglebitErrorsW<ErrStat1Spec> {
        SuccessiveSinglebitErrorsW::new(self, 4)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Level parity error Error Status"]
    #[inline(always)]
    #[must_use]
    pub fn level_parity_error(&mut self) -> LevelParityErrorW<ErrStat1Spec> {
        LevelParityErrorW::new(self, 5)
    }
    #[doc = "Bit 7 - 7:7\\]
control register error pending, Level interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn control_register_error(&mut self) -> ControlRegisterErrorW<ErrStat1Spec> {
        ControlRegisterErrorW::new(self, 7)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Clear Single Bit Error Status"]
    #[inline(always)]
    #[must_use]
    pub fn clear_single_bit(&mut self) -> ClearSingleBitW<ErrStat1Spec> {
        ClearSingleBitW::new(self, 8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Clear Double Bit Error Status"]
    #[inline(always)]
    #[must_use]
    pub fn clear_double_bit(&mut self) -> ClearDoubleBitW<ErrStat1Spec> {
        ClearDoubleBitW::new(self, 10)
    }
    #[doc = "Bit 12 - 12:12\\]
Clear other Error Status"]
    #[inline(always)]
    #[must_use]
    pub fn clear_other_error(&mut self) -> ClearOtherErrorW<ErrStat1Spec> {
        ClearOtherErrorW::new(self, 12)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Clear parity Error Status"]
    #[inline(always)]
    #[must_use]
    pub fn clear_parity_error(&mut self) -> ClearParityErrorW<ErrStat1Spec> {
        ClearParityErrorW::new(self, 13)
    }
    #[doc = "Bit 15 - 15:15\\]
Clear control reg error Error Status, you must also re write the contorl ergister itself to clear this"]
    #[inline(always)]
    #[must_use]
    pub fn clear_control_reg(&mut self) -> ClearControlRegW<ErrStat1Spec> {
        ClearControlRegW::new(self, 15)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Data bit that corresponds to the single-bit error"]
    #[inline(always)]
    #[must_use]
    pub fn data_bit_that(&mut self) -> DataBitThatW<ErrStat1Spec> {
        DataBitThatW::new(self, 16)
    }
}
#[doc = "ECC Error Status1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_stat1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_stat1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrStat1Spec;
impl crate::RegisterSpec for ErrStat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_stat1::R`](R) reader structure"]
impl crate::Readable for ErrStat1Spec {}
#[doc = "`write(|w| ..)` method takes [`err_stat1::W`](W) writer structure"]
impl crate::Writable for ErrStat1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets err_stat1 to value 0"]
impl crate::Resettable for ErrStat1Spec {
    const RESET_VALUE: u32 = 0;
}
