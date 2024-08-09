#[doc = "Register `IOLPBKTSTCR` reader"]
pub type R = crate::R<IolpbktstcrSpec>;
#[doc = "Register `IOLPBKTSTCR` writer"]
pub type W = crate::W<IolpbktstcrSpec>;
#[doc = "Field `RXPENA` reader - 0:0\\]
Module Analog loopback through Receive Pin Enable. User and Privileged mode reads. Write only in privileged mode: Write/Read : 1 = Analog loopback through receive pin 0 = Analog loopback through transmit pin. This bit is valid only when LPBK TYPE = ΓÇÿ1ΓÇÖ which chooses Analog loopback mode."]
pub type RxpenaR = crate::BitReader;
#[doc = "Field `RXPENA` writer - 0:0\\]
Module Analog loopback through Receive Pin Enable. User and Privileged mode reads. Write only in privileged mode: Write/Read : 1 = Analog loopback through receive pin 0 = Analog loopback through transmit pin. This bit is valid only when LPBK TYPE = ΓÇÿ1ΓÇÖ which chooses Analog loopback mode."]
pub type RxpenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPBKTYPE` reader - 1:1\\]
Module IO Loopback Type (Analog/Digital). User and Privileged mode reads. Write access only in Privileged mode. Write/Read : 1 = Analog loopback is enabled in module I/O DFT mode when IOLPBKTSTENA = 1010) 0 = Digital loopback is enabled in module I/O DFT mode when IOLPBKTSTENA = 1010"]
pub type LpbktypeR = crate::BitReader;
#[doc = "Field `LPBKTYPE` writer - 1:1\\]
Module IO Loopback Type (Analog/Digital). User and Privileged mode reads. Write access only in Privileged mode. Write/Read : 1 = Analog loopback is enabled in module I/O DFT mode when IOLPBKTSTENA = 1010) 0 = Digital loopback is enabled in module I/O DFT mode when IOLPBKTSTENA = 1010"]
pub type LpbktypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRLSCSPINERR` reader - 2:2\\]
Control bit to enable the injection of an error on SPISCS\\[7:0\\]
pins. Individual pins of SPISCS\\[7:0\\]
can be choosen using ERR SCS PIN. 1 = Enable the error inducing logic to the SPISCS pins. 0 = Disable the error inducing logic."]
pub type CtrlscspinerrR = crate::BitReader;
#[doc = "Field `CTRLSCSPINERR` writer - 2:2\\]
Control bit to enable the injection of an error on SPISCS\\[7:0\\]
pins. Individual pins of SPISCS\\[7:0\\]
can be choosen using ERR SCS PIN. 1 = Enable the error inducing logic to the SPISCS pins. 0 = Disable the error inducing logic."]
pub type CtrlscspinerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRSCSPIN` reader - 5:3\\]
Inject Error on ChipSelect Pin. The value in this field is decoded to find out the ChipSelect pin on which to inject an error. During the analog loopback of IO Loopback Test mode if CTRL SCS PIN ERR bit is set to ΓÇÿ1ΓÇÖ, then the chipselect pin selected by this field is forced to the opposite of its original CSNR bit. 000 - Select SPISCS\\[0\\]
for injecting error 001 - Select SPISCS\\[1\\]
for injecting error . 111 - Select SPISCS\\[7\\]
for injecting error"]
pub type ErrscspinR = crate::FieldReader;
#[doc = "Field `ERRSCSPIN` writer - 5:3\\]
Inject Error on ChipSelect Pin. The value in this field is decoded to find out the ChipSelect pin on which to inject an error. During the analog loopback of IO Loopback Test mode if CTRL SCS PIN ERR bit is set to ΓÇÿ1ΓÇÖ, then the chipselect pin selected by this field is forced to the opposite of its original CSNR bit. 000 - Select SPISCS\\[0\\]
for injecting error 001 - Select SPISCS\\[1\\]
for injecting error . 111 - Select SPISCS\\[7\\]
for injecting error"]
pub type ErrscspinW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NU1` reader - 7:6\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - 7:6\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOLPBKTSTENA` reader - 11:8\\]
Module I/O Loopback Test Enable Key User and Privileged mode reads. Write access only in Privileged mode. Write: 1010 = I/O DFT is enabled All other values = I/O DFT is disabled Read: 1010 = I/O DFT is enabled All other values = I/O DFT is disabled"]
pub type IolpbktstenaR = crate::FieldReader;
#[doc = "Field `IOLPBKTSTENA` writer - 11:8\\]
Module I/O Loopback Test Enable Key User and Privileged mode reads. Write access only in Privileged mode. Write: 1010 = I/O DFT is enabled All other values = I/O DFT is disabled Read: 1010 = I/O DFT is enabled All other values = I/O DFT is disabled"]
pub type IolpbktstenaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU2` reader - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu2R = crate::FieldReader;
#[doc = "Field `NU2` writer - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CTRLDLENERR` reader - 16:16\\]
Controls inducing of Data Length Error during IO Loopback Test mode. 1 = When in Master mode, forces the SPIENA pin(if functional) to ΓÇÿ1ΓÇÖ when the module starts Shifting the data. When in Slave mode, forces the incoming SPISCS pin(if functional) to ΓÇÿ1ΓÇÖ when the module starts shifting the data.. 0 = No affect on Data Length Error."]
pub type CtrldlenerrR = crate::BitReader;
#[doc = "Field `CTRLDLENERR` writer - 16:16\\]
Controls inducing of Data Length Error during IO Loopback Test mode. 1 = When in Master mode, forces the SPIENA pin(if functional) to ΓÇÿ1ΓÇÖ when the module starts Shifting the data. When in Slave mode, forces the incoming SPISCS pin(if functional) to ΓÇÿ1ΓÇÖ when the module starts shifting the data.. 0 = No affect on Data Length Error."]
pub type CtrldlenerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRLTIMEOUT` reader - 17:17\\]
Controls inducing of TIMEOUT Error during IO LoopbacK Test mode. 1 = Forces the incoming SPIENA pin (if functional) to remain ΓÇÿ1ΓÇÖ when transmission is initiated. The forcing will be retained until the Kernel reaches IDLE state. 0 = No affect on TIMEOUT Error."]
pub type CtrltimeoutR = crate::BitReader;
#[doc = "Field `CTRLTIMEOUT` writer - 17:17\\]
Controls inducing of TIMEOUT Error during IO LoopbacK Test mode. 1 = Forces the incoming SPIENA pin (if functional) to remain ΓÇÿ1ΓÇÖ when transmission is initiated. The forcing will be retained until the Kernel reaches IDLE state. 0 = No affect on TIMEOUT Error."]
pub type CtrltimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRLPARERR` reader - 18:18\\]
Controls inducing of Parity Error during IO Loopback Test mode. 1 = Flips the Parity Polarity signal being used for transmit parity generation logic 0 = No affect on Parity Error"]
pub type CtrlparerrR = crate::BitReader;
#[doc = "Field `CTRLPARERR` writer - 18:18\\]
Controls inducing of Parity Error during IO Loopback Test mode. 1 = Flips the Parity Polarity signal being used for transmit parity generation logic 0 = No affect on Parity Error"]
pub type CtrlparerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRLDESYNC` reader - 19:19\\]
Controls inducing of DESYNC Error during IO Loopback Test mode. 1 = Forces the incoming SPIENA pin (if functional) to remain ΓÇÿ0ΓÇÖ even after the transfer complete. This forcing will be retained until the Kernel reaches IDLE state. 0 = No affect on DESYNC Error."]
pub type CtrldesyncR = crate::BitReader;
#[doc = "Field `CTRLDESYNC` writer - 19:19\\]
Controls inducing of DESYNC Error during IO Loopback Test mode. 1 = Forces the incoming SPIENA pin (if functional) to remain ΓÇÿ0ΓÇÖ even after the transfer complete. This forcing will be retained until the Kernel reaches IDLE state. 0 = No affect on DESYNC Error."]
pub type CtrldesyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRLBITERR` reader - 20:20\\]
Controls inducing of BITERR during IO Loopback Test mode. 1 = The value of incoming data from the loopback Transmit pin is flipped. 0 = No affect on BIT ERROR."]
pub type CtrlbiterrR = crate::BitReader;
#[doc = "Field `CTRLBITERR` writer - 20:20\\]
Controls inducing of BITERR during IO Loopback Test mode. 1 = The value of incoming data from the loopback Transmit pin is flipped. 0 = No affect on BIT ERROR."]
pub type CtrlbiterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU3` reader - 23:21\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu3R = crate::FieldReader;
#[doc = "Field `NU3` writer - 23:21\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SCSFAILFLG` reader - 24:24\\]
Bit indicating a failure on SPISCS pin compare during analog loopback during IO Loopback Test mode. Read 1 = A comparison between the internal CSNR field and the analog looped back value of SPISCS\\[7:0\\]
pins failed. A stuck-at fault is detected on one of the SPISCS\\[7:0\\]. Comparison is done only on the pins which are configured as functional and during transfer operation. 0 = No miscompares on any of the 8 chipselect pin value comparison with the internal Chipselect number CSNR during transfers. Write 1 = Clear this Flag bit. 0 = No effect."]
pub type ScsfailflgR = crate::BitReader;
#[doc = "Field `SCSFAILFLG` writer - 24:24\\]
Bit indicating a failure on SPISCS pin compare during analog loopback during IO Loopback Test mode. Read 1 = A comparison between the internal CSNR field and the analog looped back value of SPISCS\\[7:0\\]
pins failed. A stuck-at fault is detected on one of the SPISCS\\[7:0\\]. Comparison is done only on the pins which are configured as functional and during transfer operation. 0 = No miscompares on any of the 8 chipselect pin value comparison with the internal Chipselect number CSNR during transfers. Write 1 = Clear this Flag bit. 0 = No effect."]
pub type ScsfailflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU4` reader - 31:25\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu4R = crate::FieldReader;
#[doc = "Field `NU4` writer - 31:25\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu4W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Module Analog loopback through Receive Pin Enable. User and Privileged mode reads. Write only in privileged mode: Write/Read : 1 = Analog loopback through receive pin 0 = Analog loopback through transmit pin. This bit is valid only when LPBK TYPE = ΓÇÿ1ΓÇÖ which chooses Analog loopback mode."]
    #[inline(always)]
    pub fn rxpena(&self) -> RxpenaR {
        RxpenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Module IO Loopback Type (Analog/Digital). User and Privileged mode reads. Write access only in Privileged mode. Write/Read : 1 = Analog loopback is enabled in module I/O DFT mode when IOLPBKTSTENA = 1010) 0 = Digital loopback is enabled in module I/O DFT mode when IOLPBKTSTENA = 1010"]
    #[inline(always)]
    pub fn lpbktype(&self) -> LpbktypeR {
        LpbktypeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Control bit to enable the injection of an error on SPISCS\\[7:0\\]
pins. Individual pins of SPISCS\\[7:0\\]
can be choosen using ERR SCS PIN. 1 = Enable the error inducing logic to the SPISCS pins. 0 = Disable the error inducing logic."]
    #[inline(always)]
    pub fn ctrlscspinerr(&self) -> CtrlscspinerrR {
        CtrlscspinerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Inject Error on ChipSelect Pin. The value in this field is decoded to find out the ChipSelect pin on which to inject an error. During the analog loopback of IO Loopback Test mode if CTRL SCS PIN ERR bit is set to ΓÇÿ1ΓÇÖ, then the chipselect pin selected by this field is forced to the opposite of its original CSNR bit. 000 - Select SPISCS\\[0\\]
for injecting error 001 - Select SPISCS\\[1\\]
for injecting error . 111 - Select SPISCS\\[7\\]
for injecting error"]
    #[inline(always)]
    pub fn errscspin(&self) -> ErrscspinR {
        ErrscspinR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Module I/O Loopback Test Enable Key User and Privileged mode reads. Write access only in Privileged mode. Write: 1010 = I/O DFT is enabled All other values = I/O DFT is disabled Read: 1010 = I/O DFT is enabled All other values = I/O DFT is disabled"]
    #[inline(always)]
    pub fn iolpbktstena(&self) -> IolpbktstenaR {
        IolpbktstenaR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Controls inducing of Data Length Error during IO Loopback Test mode. 1 = When in Master mode, forces the SPIENA pin(if functional) to ΓÇÿ1ΓÇÖ when the module starts Shifting the data. When in Slave mode, forces the incoming SPISCS pin(if functional) to ΓÇÿ1ΓÇÖ when the module starts shifting the data.. 0 = No affect on Data Length Error."]
    #[inline(always)]
    pub fn ctrldlenerr(&self) -> CtrldlenerrR {
        CtrldlenerrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Controls inducing of TIMEOUT Error during IO LoopbacK Test mode. 1 = Forces the incoming SPIENA pin (if functional) to remain ΓÇÿ1ΓÇÖ when transmission is initiated. The forcing will be retained until the Kernel reaches IDLE state. 0 = No affect on TIMEOUT Error."]
    #[inline(always)]
    pub fn ctrltimeout(&self) -> CtrltimeoutR {
        CtrltimeoutR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Controls inducing of Parity Error during IO Loopback Test mode. 1 = Flips the Parity Polarity signal being used for transmit parity generation logic 0 = No affect on Parity Error"]
    #[inline(always)]
    pub fn ctrlparerr(&self) -> CtrlparerrR {
        CtrlparerrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Controls inducing of DESYNC Error during IO Loopback Test mode. 1 = Forces the incoming SPIENA pin (if functional) to remain ΓÇÿ0ΓÇÖ even after the transfer complete. This forcing will be retained until the Kernel reaches IDLE state. 0 = No affect on DESYNC Error."]
    #[inline(always)]
    pub fn ctrldesync(&self) -> CtrldesyncR {
        CtrldesyncR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Controls inducing of BITERR during IO Loopback Test mode. 1 = The value of incoming data from the loopback Transmit pin is flipped. 0 = No affect on BIT ERROR."]
    #[inline(always)]
    pub fn ctrlbiterr(&self) -> CtrlbiterrR {
        CtrlbiterrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu3(&self) -> Nu3R {
        Nu3R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Bit indicating a failure on SPISCS pin compare during analog loopback during IO Loopback Test mode. Read 1 = A comparison between the internal CSNR field and the analog looped back value of SPISCS\\[7:0\\]
pins failed. A stuck-at fault is detected on one of the SPISCS\\[7:0\\]. Comparison is done only on the pins which are configured as functional and during transfer operation. 0 = No miscompares on any of the 8 chipselect pin value comparison with the internal Chipselect number CSNR during transfers. Write 1 = Clear this Flag bit. 0 = No effect."]
    #[inline(always)]
    pub fn scsfailflg(&self) -> ScsfailflgR {
        ScsfailflgR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu4(&self) -> Nu4R {
        Nu4R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Module Analog loopback through Receive Pin Enable. User and Privileged mode reads. Write only in privileged mode: Write/Read : 1 = Analog loopback through receive pin 0 = Analog loopback through transmit pin. This bit is valid only when LPBK TYPE = ΓÇÿ1ΓÇÖ which chooses Analog loopback mode."]
    #[inline(always)]
    #[must_use]
    pub fn rxpena(&mut self) -> RxpenaW<IolpbktstcrSpec> {
        RxpenaW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Module IO Loopback Type (Analog/Digital). User and Privileged mode reads. Write access only in Privileged mode. Write/Read : 1 = Analog loopback is enabled in module I/O DFT mode when IOLPBKTSTENA = 1010) 0 = Digital loopback is enabled in module I/O DFT mode when IOLPBKTSTENA = 1010"]
    #[inline(always)]
    #[must_use]
    pub fn lpbktype(&mut self) -> LpbktypeW<IolpbktstcrSpec> {
        LpbktypeW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Control bit to enable the injection of an error on SPISCS\\[7:0\\]
pins. Individual pins of SPISCS\\[7:0\\]
can be choosen using ERR SCS PIN. 1 = Enable the error inducing logic to the SPISCS pins. 0 = Disable the error inducing logic."]
    #[inline(always)]
    #[must_use]
    pub fn ctrlscspinerr(&mut self) -> CtrlscspinerrW<IolpbktstcrSpec> {
        CtrlscspinerrW::new(self, 2)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Inject Error on ChipSelect Pin. The value in this field is decoded to find out the ChipSelect pin on which to inject an error. During the analog loopback of IO Loopback Test mode if CTRL SCS PIN ERR bit is set to ΓÇÿ1ΓÇÖ, then the chipselect pin selected by this field is forced to the opposite of its original CSNR bit. 000 - Select SPISCS\\[0\\]
for injecting error 001 - Select SPISCS\\[1\\]
for injecting error . 111 - Select SPISCS\\[7\\]
for injecting error"]
    #[inline(always)]
    #[must_use]
    pub fn errscspin(&mut self) -> ErrscspinW<IolpbktstcrSpec> {
        ErrscspinW::new(self, 3)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IolpbktstcrSpec> {
        Nu1W::new(self, 6)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Module I/O Loopback Test Enable Key User and Privileged mode reads. Write access only in Privileged mode. Write: 1010 = I/O DFT is enabled All other values = I/O DFT is disabled Read: 1010 = I/O DFT is enabled All other values = I/O DFT is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn iolpbktstena(&mut self) -> IolpbktstenaW<IolpbktstcrSpec> {
        IolpbktstenaW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<IolpbktstcrSpec> {
        Nu2W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
Controls inducing of Data Length Error during IO Loopback Test mode. 1 = When in Master mode, forces the SPIENA pin(if functional) to ΓÇÿ1ΓÇÖ when the module starts Shifting the data. When in Slave mode, forces the incoming SPISCS pin(if functional) to ΓÇÿ1ΓÇÖ when the module starts shifting the data.. 0 = No affect on Data Length Error."]
    #[inline(always)]
    #[must_use]
    pub fn ctrldlenerr(&mut self) -> CtrldlenerrW<IolpbktstcrSpec> {
        CtrldlenerrW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Controls inducing of TIMEOUT Error during IO LoopbacK Test mode. 1 = Forces the incoming SPIENA pin (if functional) to remain ΓÇÿ1ΓÇÖ when transmission is initiated. The forcing will be retained until the Kernel reaches IDLE state. 0 = No affect on TIMEOUT Error."]
    #[inline(always)]
    #[must_use]
    pub fn ctrltimeout(&mut self) -> CtrltimeoutW<IolpbktstcrSpec> {
        CtrltimeoutW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Controls inducing of Parity Error during IO Loopback Test mode. 1 = Flips the Parity Polarity signal being used for transmit parity generation logic 0 = No affect on Parity Error"]
    #[inline(always)]
    #[must_use]
    pub fn ctrlparerr(&mut self) -> CtrlparerrW<IolpbktstcrSpec> {
        CtrlparerrW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Controls inducing of DESYNC Error during IO Loopback Test mode. 1 = Forces the incoming SPIENA pin (if functional) to remain ΓÇÿ0ΓÇÖ even after the transfer complete. This forcing will be retained until the Kernel reaches IDLE state. 0 = No affect on DESYNC Error."]
    #[inline(always)]
    #[must_use]
    pub fn ctrldesync(&mut self) -> CtrldesyncW<IolpbktstcrSpec> {
        CtrldesyncW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Controls inducing of BITERR during IO Loopback Test mode. 1 = The value of incoming data from the loopback Transmit pin is flipped. 0 = No affect on BIT ERROR."]
    #[inline(always)]
    #[must_use]
    pub fn ctrlbiterr(&mut self) -> CtrlbiterrW<IolpbktstcrSpec> {
        CtrlbiterrW::new(self, 20)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu3(&mut self) -> Nu3W<IolpbktstcrSpec> {
        Nu3W::new(self, 21)
    }
    #[doc = "Bit 24 - 24:24\\]
Bit indicating a failure on SPISCS pin compare during analog loopback during IO Loopback Test mode. Read 1 = A comparison between the internal CSNR field and the analog looped back value of SPISCS\\[7:0\\]
pins failed. A stuck-at fault is detected on one of the SPISCS\\[7:0\\]. Comparison is done only on the pins which are configured as functional and during transfer operation. 0 = No miscompares on any of the 8 chipselect pin value comparison with the internal Chipselect number CSNR during transfers. Write 1 = Clear this Flag bit. 0 = No effect."]
    #[inline(always)]
    #[must_use]
    pub fn scsfailflg(&mut self) -> ScsfailflgW<IolpbktstcrSpec> {
        ScsfailflgW::new(self, 24)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu4(&mut self) -> Nu4W<IolpbktstcrSpec> {
        Nu4W::new(self, 25)
    }
}
#[doc = "SPI/MibSPI IO Loopback Test Control Register This register controls test mode for I/O pins. It also controls whether loop-back should be digital or analog ones in this test mode. In addition it contains control bits to induce some of the error condition into the module. These are to be used for test purpose only. All the control/status bits in this register are valid only when IO LPBK TST ENA field is set to ΓÇ£1010ΓÇ¥.\n\nYou can [`read`](crate::Reg::read) this register and get [`iolpbktstcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iolpbktstcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IolpbktstcrSpec;
impl crate::RegisterSpec for IolpbktstcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iolpbktstcr::R`](R) reader structure"]
impl crate::Readable for IolpbktstcrSpec {}
#[doc = "`write(|w| ..)` method takes [`iolpbktstcr::W`](W) writer structure"]
impl crate::Writable for IolpbktstcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOLPBKTSTCR to value 0"]
impl crate::Resettable for IolpbktstcrSpec {
    const RESET_VALUE: u32 = 0;
}
