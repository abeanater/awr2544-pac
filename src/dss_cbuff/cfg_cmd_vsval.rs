#[doc = "Register `CFG_CMD_VSVAL` reader"]
pub type R = crate::R<CfgCmdVsvalSpec>;
#[doc = "Register `CFG_CMD_VSVAL` writer"]
pub type W = crate::W<CfgCmdVsvalSpec>;
#[doc = "Field `CFG_CMD_VSVAL` reader - 31:0\\]
CSI2 Programming : Configure the VSync Start Short Packet Value LVDS Programming : Configure with the static value : 0xAAAAAAAA"]
pub type CfgCmdVsvalR = crate::FieldReader<u32>;
#[doc = "Field `CFG_CMD_VSVAL` writer - 31:0\\]
CSI2 Programming : Configure the VSync Start Short Packet Value LVDS Programming : Configure with the static value : 0xAAAAAAAA"]
pub type CfgCmdVsvalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
CSI2 Programming : Configure the VSync Start Short Packet Value LVDS Programming : Configure with the static value : 0xAAAAAAAA"]
    #[inline(always)]
    pub fn cfg_cmd_vsval(&self) -> CfgCmdVsvalR {
        CfgCmdVsvalR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
CSI2 Programming : Configure the VSync Start Short Packet Value LVDS Programming : Configure with the static value : 0xAAAAAAAA"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_cmd_vsval(&mut self) -> CfgCmdVsvalW<CfgCmdVsvalSpec> {
        CfgCmdVsvalW::new(self, 0)
    }
}
#[doc = "VSYNC Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_cmd_vsval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_cmd_vsval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgCmdVsvalSpec;
impl crate::RegisterSpec for CfgCmdVsvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_cmd_vsval::R`](R) reader structure"]
impl crate::Readable for CfgCmdVsvalSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_cmd_vsval::W`](W) writer structure"]
impl crate::Writable for CfgCmdVsvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_CMD_VSVAL to value 0"]
impl crate::Resettable for CfgCmdVsvalSpec {
    const RESET_VALUE: u32 = 0;
}
