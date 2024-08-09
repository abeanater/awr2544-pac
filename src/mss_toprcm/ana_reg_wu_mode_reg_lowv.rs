#[doc = "Register `ANA_REG_WU_MODE_REG_LOWV` reader"]
pub type R = crate::R<AnaRegWuModeRegLowvSpec>;
#[doc = "Register `ANA_REG_WU_MODE_REG_LOWV` writer"]
pub type W = crate::W<AnaRegWuModeRegLowvSpec>;
#[doc = "Field `FUNC_TEST_DET_SYNC` reader - 0:0\\]
Latched Output of Functional Test Mode SOP"]
pub type FuncTestDetSyncR = crate::BitReader;
#[doc = "Field `FUNC_TEST_DET_SYNC` writer - 0:0\\]
Latched Output of Functional Test Mode SOP"]
pub type FuncTestDetSyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEST_MODE_DET_SYNC` reader - 1:1\\]
Latched Output of Test Mode Detect SOP"]
pub type TestModeDetSyncR = crate::BitReader;
#[doc = "Field `TEST_MODE_DET_SYNC` writer - 1:1\\]
Latched Output of Test Mode Detect SOP"]
pub type TestModeDetSyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOP_MODE_LAT_4_0` reader - 6:2\\]
SOP Mode Latched Output Bit2,Bit1,Bit0: 010 SOP_MODE1 SCAN/ATPG 011 SOP_MODE2 DEV/FLED/ORBIT 000 SOP_MODE3 THB 001 SOP_MODE4 FUNC 101 SOP_MODE5 DEV MANAGEMENT 110 SOP_MODE6 DEVBOOT 111 SOP_MODE7 FUNC_SFLASH_RESET Bit4,Bit3: 00 40 MHz 11 50 MHz"]
pub type SopModeLat4_0R = crate::FieldReader;
#[doc = "Field `SOP_MODE_LAT_4_0` writer - 6:2\\]
SOP Mode Latched Output Bit2,Bit1,Bit0: 010 SOP_MODE1 SCAN/ATPG 011 SOP_MODE2 DEV/FLED/ORBIT 000 SOP_MODE3 THB 001 SOP_MODE4 FUNC 101 SOP_MODE5 DEV MANAGEMENT 110 SOP_MODE6 DEVBOOT 111 SOP_MODE7 FUNC_SFLASH_RESET Bit4,Bit3: 00 40 MHz 11 50 MHz"]
pub type SopModeLat4_0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `Reserved0` reader - 31:7\\]
Reserved Reads return 0x0 and writes have no effect. 0x0000000 = Functional Reset"]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `Reserved0` writer - 31:7\\]
Reserved Reads return 0x0 and writes have no effect. 0x0000000 = Functional Reset"]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Latched Output of Functional Test Mode SOP"]
    #[inline(always)]
    pub fn func_test_det_sync(&self) -> FuncTestDetSyncR {
        FuncTestDetSyncR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Latched Output of Test Mode Detect SOP"]
    #[inline(always)]
    pub fn test_mode_det_sync(&self) -> TestModeDetSyncR {
        TestModeDetSyncR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - 6:2\\]
SOP Mode Latched Output Bit2,Bit1,Bit0: 010 SOP_MODE1 SCAN/ATPG 011 SOP_MODE2 DEV/FLED/ORBIT 000 SOP_MODE3 THB 001 SOP_MODE4 FUNC 101 SOP_MODE5 DEV MANAGEMENT 110 SOP_MODE6 DEVBOOT 111 SOP_MODE7 FUNC_SFLASH_RESET Bit4,Bit3: 00 40 MHz 11 50 MHz"]
    #[inline(always)]
    pub fn sop_mode_lat_4_0(&self) -> SopModeLat4_0R {
        SopModeLat4_0R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Reserved Reads return 0x0 and writes have no effect. 0x0000000 = Functional Reset"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Latched Output of Functional Test Mode SOP"]
    #[inline(always)]
    #[must_use]
    pub fn func_test_det_sync(&mut self) -> FuncTestDetSyncW<AnaRegWuModeRegLowvSpec> {
        FuncTestDetSyncW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Latched Output of Test Mode Detect SOP"]
    #[inline(always)]
    #[must_use]
    pub fn test_mode_det_sync(&mut self) -> TestModeDetSyncW<AnaRegWuModeRegLowvSpec> {
        TestModeDetSyncW::new(self, 1)
    }
    #[doc = "Bits 2:6 - 6:2\\]
SOP Mode Latched Output Bit2,Bit1,Bit0: 010 SOP_MODE1 SCAN/ATPG 011 SOP_MODE2 DEV/FLED/ORBIT 000 SOP_MODE3 THB 001 SOP_MODE4 FUNC 101 SOP_MODE5 DEV MANAGEMENT 110 SOP_MODE6 DEVBOOT 111 SOP_MODE7 FUNC_SFLASH_RESET Bit4,Bit3: 00 40 MHz 11 50 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn sop_mode_lat_4_0(&mut self) -> SopModeLat4_0W<AnaRegWuModeRegLowvSpec> {
        SopModeLat4_0W::new(self, 2)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Reserved Reads return 0x0 and writes have no effect. 0x0000000 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<AnaRegWuModeRegLowvSpec> {
        Reserved0W::new(self, 7)
    }
}
#[doc = "ANA_REG_WU_MODE_REG_LOWV\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_wu_mode_reg_lowv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_wu_mode_reg_lowv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaRegWuModeRegLowvSpec;
impl crate::RegisterSpec for AnaRegWuModeRegLowvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_reg_wu_mode_reg_lowv::R`](R) reader structure"]
impl crate::Readable for AnaRegWuModeRegLowvSpec {}
#[doc = "`write(|w| ..)` method takes [`ana_reg_wu_mode_reg_lowv::W`](W) writer structure"]
impl crate::Writable for AnaRegWuModeRegLowvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANA_REG_WU_MODE_REG_LOWV to value 0"]
impl crate::Resettable for AnaRegWuModeRegLowvSpec {
    const RESET_VALUE: u32 = 0;
}
