#[doc = "Register `CFG_LVDS_GEN_2` reader"]
pub type R = crate::R<CfgLvdsGen2Spec>;
#[doc = "Register `CFG_LVDS_GEN_2` writer"]
pub type W = crate::W<CfgLvdsGen2Spec>;
#[doc = "Field `CFG_LVDS_GEN_2` reader - 31:0\\]
CFG_LVDS_GEN_2\\[0\\]: Configure LSB/MSB first for CRC. This feature is supported only when the field CFG_LVDS_GEN_0\\[28\\]
is set to 1 0 -> The calculated value of 32-bit Ethernet polynomial CRC is swapped and sent out, clear this bit if data is set to LSB first (CFG_LVDS_GEN_0\\[23\\]=0) but CRC should be MSB first or vice-versa 1 -> The calculated value of 32-bit Ethernet polynomial CRC is sent out without swapping, set this bit if both data and CRC should have same format (LSB/MSB first) CFG_LVDS_GEN_2\\[1\\]: Configure value of frame clock during inter frame period 0 -> Frame clock is held low 1 -> Frame clock is held high CFG_LVDS_GEN_2\\[2\\]: Configure frame clock period. This feature is supported only when the field CFG_LVDS_GEN_0\\[28\\]
is set to 1 0 -> 32-bit CRC is transmitted as single packet with frame clock set to 16h16l (16 high 16 low) configuration 1 -> 32-bit CRC is transmitted as two packets with frame clock set to 8h8l (8 high 8 low) configuration for each packet CFG_LVDS_GEN_2\\[3\\]: Configure bit clock during inter frame period 0 -> Bit clock toggles during inter frame period 1 -> Bit clock does not toggle during inter frame period, the value of bit clock is held low This feature is supported when DDR clock is selected (CFG_LVDS_GEN_0\\[10\\]=1) and first data sample is driven on posedge of DDR clock (CFG_LVDS_GEN_0\\[22\\]=1) CFG_LVDS_GEN_2\\[4\\]: Configure CRC inversion. This feature is supported only when the field CFG_LVDS_GEN_0\\[28\\]
is set to 1 0 -> The calculated value of 32-bit Ethernet polynomial CRC is inverted and sent out 1 -> The calculated value of 32-bit Ethernet polynomial CRC is sent out without inversion CFG_LVDS_GEN_2\\[5\\]: Enable/disable the calibration mode, in this mode frame clock will follow data lane\\[0\\]
0 -> Calibration mode is disabled 1 -> Calibration mode is enabled"]
pub type CfgLvdsGen2R = crate::FieldReader<u32>;
#[doc = "Field `CFG_LVDS_GEN_2` writer - 31:0\\]
CFG_LVDS_GEN_2\\[0\\]: Configure LSB/MSB first for CRC. This feature is supported only when the field CFG_LVDS_GEN_0\\[28\\]
is set to 1 0 -> The calculated value of 32-bit Ethernet polynomial CRC is swapped and sent out, clear this bit if data is set to LSB first (CFG_LVDS_GEN_0\\[23\\]=0) but CRC should be MSB first or vice-versa 1 -> The calculated value of 32-bit Ethernet polynomial CRC is sent out without swapping, set this bit if both data and CRC should have same format (LSB/MSB first) CFG_LVDS_GEN_2\\[1\\]: Configure value of frame clock during inter frame period 0 -> Frame clock is held low 1 -> Frame clock is held high CFG_LVDS_GEN_2\\[2\\]: Configure frame clock period. This feature is supported only when the field CFG_LVDS_GEN_0\\[28\\]
is set to 1 0 -> 32-bit CRC is transmitted as single packet with frame clock set to 16h16l (16 high 16 low) configuration 1 -> 32-bit CRC is transmitted as two packets with frame clock set to 8h8l (8 high 8 low) configuration for each packet CFG_LVDS_GEN_2\\[3\\]: Configure bit clock during inter frame period 0 -> Bit clock toggles during inter frame period 1 -> Bit clock does not toggle during inter frame period, the value of bit clock is held low This feature is supported when DDR clock is selected (CFG_LVDS_GEN_0\\[10\\]=1) and first data sample is driven on posedge of DDR clock (CFG_LVDS_GEN_0\\[22\\]=1) CFG_LVDS_GEN_2\\[4\\]: Configure CRC inversion. This feature is supported only when the field CFG_LVDS_GEN_0\\[28\\]
is set to 1 0 -> The calculated value of 32-bit Ethernet polynomial CRC is inverted and sent out 1 -> The calculated value of 32-bit Ethernet polynomial CRC is sent out without inversion CFG_LVDS_GEN_2\\[5\\]: Enable/disable the calibration mode, in this mode frame clock will follow data lane\\[0\\]
0 -> Calibration mode is disabled 1 -> Calibration mode is enabled"]
pub type CfgLvdsGen2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
CFG_LVDS_GEN_2\\[0\\]: Configure LSB/MSB first for CRC. This feature is supported only when the field CFG_LVDS_GEN_0\\[28\\]
is set to 1 0 -> The calculated value of 32-bit Ethernet polynomial CRC is swapped and sent out, clear this bit if data is set to LSB first (CFG_LVDS_GEN_0\\[23\\]=0) but CRC should be MSB first or vice-versa 1 -> The calculated value of 32-bit Ethernet polynomial CRC is sent out without swapping, set this bit if both data and CRC should have same format (LSB/MSB first) CFG_LVDS_GEN_2\\[1\\]: Configure value of frame clock during inter frame period 0 -> Frame clock is held low 1 -> Frame clock is held high CFG_LVDS_GEN_2\\[2\\]: Configure frame clock period. This feature is supported only when the field CFG_LVDS_GEN_0\\[28\\]
is set to 1 0 -> 32-bit CRC is transmitted as single packet with frame clock set to 16h16l (16 high 16 low) configuration 1 -> 32-bit CRC is transmitted as two packets with frame clock set to 8h8l (8 high 8 low) configuration for each packet CFG_LVDS_GEN_2\\[3\\]: Configure bit clock during inter frame period 0 -> Bit clock toggles during inter frame period 1 -> Bit clock does not toggle during inter frame period, the value of bit clock is held low This feature is supported when DDR clock is selected (CFG_LVDS_GEN_0\\[10\\]=1) and first data sample is driven on posedge of DDR clock (CFG_LVDS_GEN_0\\[22\\]=1) CFG_LVDS_GEN_2\\[4\\]: Configure CRC inversion. This feature is supported only when the field CFG_LVDS_GEN_0\\[28\\]
is set to 1 0 -> The calculated value of 32-bit Ethernet polynomial CRC is inverted and sent out 1 -> The calculated value of 32-bit Ethernet polynomial CRC is sent out without inversion CFG_LVDS_GEN_2\\[5\\]: Enable/disable the calibration mode, in this mode frame clock will follow data lane\\[0\\]
0 -> Calibration mode is disabled 1 -> Calibration mode is enabled"]
    #[inline(always)]
    pub fn cfg_lvds_gen_2(&self) -> CfgLvdsGen2R {
        CfgLvdsGen2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
CFG_LVDS_GEN_2\\[0\\]: Configure LSB/MSB first for CRC. This feature is supported only when the field CFG_LVDS_GEN_0\\[28\\]
is set to 1 0 -> The calculated value of 32-bit Ethernet polynomial CRC is swapped and sent out, clear this bit if data is set to LSB first (CFG_LVDS_GEN_0\\[23\\]=0) but CRC should be MSB first or vice-versa 1 -> The calculated value of 32-bit Ethernet polynomial CRC is sent out without swapping, set this bit if both data and CRC should have same format (LSB/MSB first) CFG_LVDS_GEN_2\\[1\\]: Configure value of frame clock during inter frame period 0 -> Frame clock is held low 1 -> Frame clock is held high CFG_LVDS_GEN_2\\[2\\]: Configure frame clock period. This feature is supported only when the field CFG_LVDS_GEN_0\\[28\\]
is set to 1 0 -> 32-bit CRC is transmitted as single packet with frame clock set to 16h16l (16 high 16 low) configuration 1 -> 32-bit CRC is transmitted as two packets with frame clock set to 8h8l (8 high 8 low) configuration for each packet CFG_LVDS_GEN_2\\[3\\]: Configure bit clock during inter frame period 0 -> Bit clock toggles during inter frame period 1 -> Bit clock does not toggle during inter frame period, the value of bit clock is held low This feature is supported when DDR clock is selected (CFG_LVDS_GEN_0\\[10\\]=1) and first data sample is driven on posedge of DDR clock (CFG_LVDS_GEN_0\\[22\\]=1) CFG_LVDS_GEN_2\\[4\\]: Configure CRC inversion. This feature is supported only when the field CFG_LVDS_GEN_0\\[28\\]
is set to 1 0 -> The calculated value of 32-bit Ethernet polynomial CRC is inverted and sent out 1 -> The calculated value of 32-bit Ethernet polynomial CRC is sent out without inversion CFG_LVDS_GEN_2\\[5\\]: Enable/disable the calibration mode, in this mode frame clock will follow data lane\\[0\\]
0 -> Calibration mode is disabled 1 -> Calibration mode is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_lvds_gen_2(&mut self) -> CfgLvdsGen2W<CfgLvdsGen2Spec> {
        CfgLvdsGen2W::new(self, 0)
    }
}
#[doc = "CFG_LVDS_GEN_2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_lvds_gen_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_lvds_gen_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgLvdsGen2Spec;
impl crate::RegisterSpec for CfgLvdsGen2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_lvds_gen_2::R`](R) reader structure"]
impl crate::Readable for CfgLvdsGen2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_lvds_gen_2::W`](W) writer structure"]
impl crate::Writable for CfgLvdsGen2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_LVDS_GEN_2 to value 0"]
impl crate::Resettable for CfgLvdsGen2Spec {
    const RESET_VALUE: u32 = 0;
}
