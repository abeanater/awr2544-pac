#[doc = "Register `CFG_CMD_VEVAL` reader"]
pub type R = crate::R<CfgCmdVevalSpec>;
#[doc = "Register `CFG_CMD_VEVAL` writer"]
pub type W = crate::W<CfgCmdVevalSpec>;
#[doc = "Field `CFG_CMD_VEVAL` reader - 31:0\\]
CSI2 Programming : Configure the VSync End Short Packet Value LVDS Programming : Configure with the static value : 0xAAAAAAAA"]
pub type CfgCmdVevalR = crate::FieldReader<u32>;
#[doc = "Field `CFG_CMD_VEVAL` writer - 31:0\\]
CSI2 Programming : Configure the VSync End Short Packet Value LVDS Programming : Configure with the static value : 0xAAAAAAAA"]
pub type CfgCmdVevalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
CSI2 Programming : Configure the VSync End Short Packet Value LVDS Programming : Configure with the static value : 0xAAAAAAAA"]
    #[inline(always)]
    pub fn cfg_cmd_veval(&self) -> CfgCmdVevalR {
        CfgCmdVevalR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
CSI2 Programming : Configure the VSync End Short Packet Value LVDS Programming : Configure with the static value : 0xAAAAAAAA"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_cmd_veval(&mut self) -> CfgCmdVevalW<CfgCmdVevalSpec> {
        CfgCmdVevalW::new(self, 0)
    }
}
#[doc = "VEND Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_cmd_veval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_cmd_veval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgCmdVevalSpec;
impl crate::RegisterSpec for CfgCmdVevalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_cmd_veval::R`](R) reader structure"]
impl crate::Readable for CfgCmdVevalSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_cmd_veval::W`](W) writer structure"]
impl crate::Writable for CfgCmdVevalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_CMD_VEVAL to value 0"]
impl crate::Resettable for CfgCmdVevalSpec {
    const RESET_VALUE: u32 = 0;
}
