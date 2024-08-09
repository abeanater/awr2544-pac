#[doc = "Register `CFG_CMD_HEVAL` reader"]
pub type R = crate::R<CfgCmdHevalSpec>;
#[doc = "Register `CFG_CMD_HEVAL` writer"]
pub type W = crate::W<CfgCmdHevalSpec>;
#[doc = "Field `CFG_CMD_HEVAL` reader - 31:0\\]
CSI2 Programming : Configure the HSync End Short Packet Value LVDS Programming : If LVDS CRC is enabled : Configure with the static value : 0x33333333 If LVDS CRC is disbaled : Configure with the static value : 0xAAAAAAAA"]
pub type CfgCmdHevalR = crate::FieldReader<u32>;
#[doc = "Field `CFG_CMD_HEVAL` writer - 31:0\\]
CSI2 Programming : Configure the HSync End Short Packet Value LVDS Programming : If LVDS CRC is enabled : Configure with the static value : 0x33333333 If LVDS CRC is disbaled : Configure with the static value : 0xAAAAAAAA"]
pub type CfgCmdHevalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
CSI2 Programming : Configure the HSync End Short Packet Value LVDS Programming : If LVDS CRC is enabled : Configure with the static value : 0x33333333 If LVDS CRC is disbaled : Configure with the static value : 0xAAAAAAAA"]
    #[inline(always)]
    pub fn cfg_cmd_heval(&self) -> CfgCmdHevalR {
        CfgCmdHevalR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
CSI2 Programming : Configure the HSync End Short Packet Value LVDS Programming : If LVDS CRC is enabled : Configure with the static value : 0x33333333 If LVDS CRC is disbaled : Configure with the static value : 0xAAAAAAAA"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_cmd_heval(&mut self) -> CfgCmdHevalW<CfgCmdHevalSpec> {
        CfgCmdHevalW::new(self, 0)
    }
}
#[doc = "HEND Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_cmd_heval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_cmd_heval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgCmdHevalSpec;
impl crate::RegisterSpec for CfgCmdHevalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_cmd_heval::R`](R) reader structure"]
impl crate::Readable for CfgCmdHevalSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_cmd_heval::W`](W) writer structure"]
impl crate::Writable for CfgCmdHevalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_CMD_HEVAL to value 0"]
impl crate::Resettable for CfgCmdHevalSpec {
    const RESET_VALUE: u32 = 0;
}
