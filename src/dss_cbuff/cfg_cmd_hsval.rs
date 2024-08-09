#[doc = "Register `CFG_CMD_HSVAL` reader"]
pub type R = crate::R<CfgCmdHsvalSpec>;
#[doc = "Register `CFG_CMD_HSVAL` writer"]
pub type W = crate::W<CfgCmdHsvalSpec>;
#[doc = "Field `CFG_CMD_HSVAL` reader - 31:0\\]
CSI2 Programming : Configure the HSync Start Short Packet Value LVDS Programming : If LVDS CRC is enabled : Configure with the static value : 0x55555555 If LVDS CRC is disbaled : Configure with the static value : 0xAAAAAAAA"]
pub type CfgCmdHsvalR = crate::FieldReader<u32>;
#[doc = "Field `CFG_CMD_HSVAL` writer - 31:0\\]
CSI2 Programming : Configure the HSync Start Short Packet Value LVDS Programming : If LVDS CRC is enabled : Configure with the static value : 0x55555555 If LVDS CRC is disbaled : Configure with the static value : 0xAAAAAAAA"]
pub type CfgCmdHsvalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
CSI2 Programming : Configure the HSync Start Short Packet Value LVDS Programming : If LVDS CRC is enabled : Configure with the static value : 0x55555555 If LVDS CRC is disbaled : Configure with the static value : 0xAAAAAAAA"]
    #[inline(always)]
    pub fn cfg_cmd_hsval(&self) -> CfgCmdHsvalR {
        CfgCmdHsvalR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
CSI2 Programming : Configure the HSync Start Short Packet Value LVDS Programming : If LVDS CRC is enabled : Configure with the static value : 0x55555555 If LVDS CRC is disbaled : Configure with the static value : 0xAAAAAAAA"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_cmd_hsval(&mut self) -> CfgCmdHsvalW<CfgCmdHsvalSpec> {
        CfgCmdHsvalW::new(self, 0)
    }
}
#[doc = "HSYNC Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_cmd_hsval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_cmd_hsval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgCmdHsvalSpec;
impl crate::RegisterSpec for CfgCmdHsvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_cmd_hsval::R`](R) reader structure"]
impl crate::Readable for CfgCmdHsvalSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_cmd_hsval::W`](W) writer structure"]
impl crate::Writable for CfgCmdHsvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_CMD_HSVAL to value 0"]
impl crate::Resettable for CfgCmdHsvalSpec {
    const RESET_VALUE: u32 = 0;
}
