#[doc = "Register `ADCBUFCFG1` reader"]
pub type R = crate::R<Adcbufcfg1Spec>;
#[doc = "Register `ADCBUFCFG1` writer"]
pub type W = crate::W<Adcbufcfg1Spec>;
#[doc = "Field `ADCBUFWRSOURCE` reader - 0:0\\]
TI Internal Feature Write source for ADC Buffer. 0 --> DFE, 1 --> VIN"]
pub type AdcbufwrsourceR = crate::BitReader;
#[doc = "Field `ADCBUFWRSOURCE` writer - 0:0\\]
TI Internal Feature Write source for ADC Buffer. 0 --> DFE, 1 --> VIN"]
pub type AdcbufwrsourceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCBUFPIPOSELINV` reader - 1:1\\]
TI Internal Feature Inversion control for ADC Buffer Ping-pong select. By default ADC Buffer write starts with Pong write. By setting this bit to 1, it will start from Ping write after reset."]
pub type AdcbufpiposelinvR = crate::BitReader;
#[doc = "Field `ADCBUFPIPOSELINV` writer - 1:1\\]
TI Internal Feature Inversion control for ADC Buffer Ping-pong select. By default ADC Buffer write starts with Pong write. By setting this bit to 1, it will start from Ping write after reset."]
pub type AdcbufpiposelinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCBUFREALONLYMODE` reader - 2:2\\]
0-->Complex Data mode, 1-->Real data mode"]
pub type AdcbufrealonlymodeR = crate::BitReader;
#[doc = "Field `ADCBUFREALONLYMODE` writer - 2:2\\]
0-->Complex Data mode, 1-->Real data mode"]
pub type AdcbufrealonlymodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCBUFRLMODECHSEL` reader - 3:3\\]
TI Internal Feature 0 -->I channel, 1 --> Q channel"]
pub type AdcbufrlmodechselR = crate::BitReader;
#[doc = "Field `ADCBUFRLMODECHSEL` writer - 3:3\\]
TI Internal Feature 0 -->I channel, 1 --> Q channel"]
pub type AdcbufrlmodechselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCBUFRL2CHINTRL` reader - 4:4\\]
TI reserved field. Do not touch"]
pub type Adcbufrl2chintrlR = crate::BitReader;
#[doc = "Field `ADCBUFRL2CHINTRL` writer - 4:4\\]
TI reserved field. Do not touch"]
pub type Adcbufrl2chintrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCBUFIQSWAP` reader - 5:5\\]
0 --> I is stored in LSB and Q is stored in MSB 1 --> Q is stored in LSB and I is stored in MSB"]
pub type AdcbufiqswapR = crate::BitReader;
#[doc = "Field `ADCBUFIQSWAP` writer - 5:5\\]
0 --> I is stored in LSB and Q is stored in MSB 1 --> Q is stored in LSB and I is stored in MSB"]
pub type AdcbufiqswapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX0EN` reader - 6:6\\]
Enable for Rx0 write"]
pub type Rx0enR = crate::BitReader;
#[doc = "Field `RX0EN` writer - 6:6\\]
Enable for Rx0 write"]
pub type Rx0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX1EN` reader - 7:7\\]
Enable for Rx1 write"]
pub type Rx1enR = crate::BitReader;
#[doc = "Field `RX1EN` writer - 7:7\\]
Enable for Rx1 write"]
pub type Rx1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX2EN` reader - 8:8\\]
Enable for Rx2 write"]
pub type Rx2enR = crate::BitReader;
#[doc = "Field `RX2EN` writer - 8:8\\]
Enable for Rx2 write"]
pub type Rx2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX3EN` reader - 9:9\\]
Enable for Rx3 write"]
pub type Rx3enR = crate::BitReader;
#[doc = "Field `RX3EN` writer - 9:9\\]
Enable for Rx3 write"]
pub type Rx3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCBUFPIPOOVRCNT` reader - 10:10\\]
TI Internal Feature Override control for ADC Buffer Ping Pong select"]
pub type AdcbufpipoovrcntR = crate::BitReader;
#[doc = "Field `ADCBUFPIPOOVRCNT` writer - 10:10\\]
TI Internal Feature Override control for ADC Buffer Ping Pong select"]
pub type AdcbufpipoovrcntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCBUFPIPOOVRVAL` reader - 11:11\\]
TI Internal Feature SW override value for ADC Buffer Ping Pong select"]
pub type AdcbufpipoovrvalR = crate::BitReader;
#[doc = "Field `ADCBUFPIPOOVRVAL` writer - 11:11\\]
TI Internal Feature SW override value for ADC Buffer Ping Pong select"]
pub type AdcbufpipoovrvalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCBUFWRITEMODE` reader - 12:12\\]
This needs to be programmed to 0x1 in AR16xx 0 --> Interleaved, 1 --> Non-interleaved"]
pub type AdcbufwritemodeR = crate::BitReader;
#[doc = "Field `ADCBUFWRITEMODE` writer - 12:12\\]
This needs to be programmed to 0x1 in AR16xx 0 --> Interleaved, 1 --> Non-interleaved"]
pub type AdcbufwritemodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCBUFCONTMODEEN` reader - 13:13\\]
Continuous mode enable for ADC Buffer. This is set when a fixed number of samples have to be stored in Ping/Pong and not depend on Chirp time-lines (Eg: Analog Lab characterization to stream out continuous data from DFE). Continous mode is expected to be only used for CZ and ADC Buffer Testpattern mode"]
pub type AdcbufcontmodeenR = crate::BitReader;
#[doc = "Field `ADCBUFCONTMODEEN` writer - 13:13\\]
Continuous mode enable for ADC Buffer. This is set when a fixed number of samples have to be stored in Ping/Pong and not depend on Chirp time-lines (Eg: Analog Lab characterization to stream out continuous data from DFE). Continous mode is expected to be only used for CZ and ADC Buffer Testpattern mode"]
pub type AdcbufcontmodeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCBUFCONTSTRTPL` reader - 14:14\\]
Start Pulse for Continuous mode. The data capture will start from Address 0 once this register is set. All the other configurations like Enable, Sample Count are expected to be programmed before this pulse. Continous mode is expected to be only used for CZ and ADC Buffer Testpattern mode : Its a wspecial access type, write to this field will generate a pulse"]
pub type AdcbufcontstrtplR = crate::BitReader;
#[doc = "Field `ADCBUFCONTSTRTPL` writer - 14:14\\]
Start Pulse for Continuous mode. The data capture will start from Address 0 once this register is set. All the other configurations like Enable, Sample Count are expected to be programmed before this pulse. Continous mode is expected to be only used for CZ and ADC Buffer Testpattern mode : Its a wspecial access type, write to this field will generate a pulse"]
pub type AdcbufcontstrtplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCBUFCONTSTOPPL` reader - 15:15\\]
Stop Pulse for Continuous mode. The data capture will stop once this register is set. Continous mode is expected to be only used for CZ and ADC Buffer Testpattern mode : Its a wspecial access type, write to this field will generate a pulse"]
pub type AdcbufcontstopplR = crate::BitReader;
#[doc = "Field `ADCBUFCONTSTOPPL` writer - 15:15\\]
Stop Pulse for Continuous mode. The data capture will stop once this register is set. Continous mode is expected to be only used for CZ and ADC Buffer Testpattern mode : Its a wspecial access type, write to this field will generate a pulse"]
pub type AdcbufcontstopplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCBUFPIPOSEL` reader - 16:16\\]
TI Internal Feature Ping-pong select value from ADC Buffer Packing logic. Even in SW override mode, this register will indicate the ping-pong select signal generated from the ADC Buffer Packing logic and not the override value."]
pub type AdcbufpiposelR = crate::BitReader;
#[doc = "Field `ADCBUFPIPOSEL` writer - 16:16\\]
TI Internal Feature Ping-pong select value from ADC Buffer Packing logic. Even in SW override mode, this register will indicate the ping-pong select signal generated from the ADC Buffer Packing logic and not the override value."]
pub type AdcbufpiposelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
TI Internal Feature Write source for ADC Buffer. 0 --> DFE, 1 --> VIN"]
    #[inline(always)]
    pub fn adcbufwrsource(&self) -> AdcbufwrsourceR {
        AdcbufwrsourceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TI Internal Feature Inversion control for ADC Buffer Ping-pong select. By default ADC Buffer write starts with Pong write. By setting this bit to 1, it will start from Ping write after reset."]
    #[inline(always)]
    pub fn adcbufpiposelinv(&self) -> AdcbufpiposelinvR {
        AdcbufpiposelinvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0-->Complex Data mode, 1-->Real data mode"]
    #[inline(always)]
    pub fn adcbufrealonlymode(&self) -> AdcbufrealonlymodeR {
        AdcbufrealonlymodeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
TI Internal Feature 0 -->I channel, 1 --> Q channel"]
    #[inline(always)]
    pub fn adcbufrlmodechsel(&self) -> AdcbufrlmodechselR {
        AdcbufrlmodechselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
TI reserved field. Do not touch"]
    #[inline(always)]
    pub fn adcbufrl2chintrl(&self) -> Adcbufrl2chintrlR {
        Adcbufrl2chintrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
0 --> I is stored in LSB and Q is stored in MSB 1 --> Q is stored in LSB and I is stored in MSB"]
    #[inline(always)]
    pub fn adcbufiqswap(&self) -> AdcbufiqswapR {
        AdcbufiqswapR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable for Rx0 write"]
    #[inline(always)]
    pub fn rx0en(&self) -> Rx0enR {
        Rx0enR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Enable for Rx1 write"]
    #[inline(always)]
    pub fn rx1en(&self) -> Rx1enR {
        Rx1enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable for Rx2 write"]
    #[inline(always)]
    pub fn rx2en(&self) -> Rx2enR {
        Rx2enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable for Rx3 write"]
    #[inline(always)]
    pub fn rx3en(&self) -> Rx3enR {
        Rx3enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
TI Internal Feature Override control for ADC Buffer Ping Pong select"]
    #[inline(always)]
    pub fn adcbufpipoovrcnt(&self) -> AdcbufpipoovrcntR {
        AdcbufpipoovrcntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
TI Internal Feature SW override value for ADC Buffer Ping Pong select"]
    #[inline(always)]
    pub fn adcbufpipoovrval(&self) -> AdcbufpipoovrvalR {
        AdcbufpipoovrvalR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
This needs to be programmed to 0x1 in AR16xx 0 --> Interleaved, 1 --> Non-interleaved"]
    #[inline(always)]
    pub fn adcbufwritemode(&self) -> AdcbufwritemodeR {
        AdcbufwritemodeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Continuous mode enable for ADC Buffer. This is set when a fixed number of samples have to be stored in Ping/Pong and not depend on Chirp time-lines (Eg: Analog Lab characterization to stream out continuous data from DFE). Continous mode is expected to be only used for CZ and ADC Buffer Testpattern mode"]
    #[inline(always)]
    pub fn adcbufcontmodeen(&self) -> AdcbufcontmodeenR {
        AdcbufcontmodeenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Start Pulse for Continuous mode. The data capture will start from Address 0 once this register is set. All the other configurations like Enable, Sample Count are expected to be programmed before this pulse. Continous mode is expected to be only used for CZ and ADC Buffer Testpattern mode : Its a wspecial access type, write to this field will generate a pulse"]
    #[inline(always)]
    pub fn adcbufcontstrtpl(&self) -> AdcbufcontstrtplR {
        AdcbufcontstrtplR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Stop Pulse for Continuous mode. The data capture will stop once this register is set. Continous mode is expected to be only used for CZ and ADC Buffer Testpattern mode : Its a wspecial access type, write to this field will generate a pulse"]
    #[inline(always)]
    pub fn adcbufcontstoppl(&self) -> AdcbufcontstopplR {
        AdcbufcontstopplR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
TI Internal Feature Ping-pong select value from ADC Buffer Packing logic. Even in SW override mode, this register will indicate the ping-pong select signal generated from the ADC Buffer Packing logic and not the override value."]
    #[inline(always)]
    pub fn adcbufpiposel(&self) -> AdcbufpiposelR {
        AdcbufpiposelR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
TI Internal Feature Write source for ADC Buffer. 0 --> DFE, 1 --> VIN"]
    #[inline(always)]
    #[must_use]
    pub fn adcbufwrsource(&mut self) -> AdcbufwrsourceW<Adcbufcfg1Spec> {
        AdcbufwrsourceW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TI Internal Feature Inversion control for ADC Buffer Ping-pong select. By default ADC Buffer write starts with Pong write. By setting this bit to 1, it will start from Ping write after reset."]
    #[inline(always)]
    #[must_use]
    pub fn adcbufpiposelinv(&mut self) -> AdcbufpiposelinvW<Adcbufcfg1Spec> {
        AdcbufpiposelinvW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
0-->Complex Data mode, 1-->Real data mode"]
    #[inline(always)]
    #[must_use]
    pub fn adcbufrealonlymode(&mut self) -> AdcbufrealonlymodeW<Adcbufcfg1Spec> {
        AdcbufrealonlymodeW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
TI Internal Feature 0 -->I channel, 1 --> Q channel"]
    #[inline(always)]
    #[must_use]
    pub fn adcbufrlmodechsel(&mut self) -> AdcbufrlmodechselW<Adcbufcfg1Spec> {
        AdcbufrlmodechselW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
TI reserved field. Do not touch"]
    #[inline(always)]
    #[must_use]
    pub fn adcbufrl2chintrl(&mut self) -> Adcbufrl2chintrlW<Adcbufcfg1Spec> {
        Adcbufrl2chintrlW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
0 --> I is stored in LSB and Q is stored in MSB 1 --> Q is stored in LSB and I is stored in MSB"]
    #[inline(always)]
    #[must_use]
    pub fn adcbufiqswap(&mut self) -> AdcbufiqswapW<Adcbufcfg1Spec> {
        AdcbufiqswapW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable for Rx0 write"]
    #[inline(always)]
    #[must_use]
    pub fn rx0en(&mut self) -> Rx0enW<Adcbufcfg1Spec> {
        Rx0enW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Enable for Rx1 write"]
    #[inline(always)]
    #[must_use]
    pub fn rx1en(&mut self) -> Rx1enW<Adcbufcfg1Spec> {
        Rx1enW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable for Rx2 write"]
    #[inline(always)]
    #[must_use]
    pub fn rx2en(&mut self) -> Rx2enW<Adcbufcfg1Spec> {
        Rx2enW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Enable for Rx3 write"]
    #[inline(always)]
    #[must_use]
    pub fn rx3en(&mut self) -> Rx3enW<Adcbufcfg1Spec> {
        Rx3enW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
TI Internal Feature Override control for ADC Buffer Ping Pong select"]
    #[inline(always)]
    #[must_use]
    pub fn adcbufpipoovrcnt(&mut self) -> AdcbufpipoovrcntW<Adcbufcfg1Spec> {
        AdcbufpipoovrcntW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
TI Internal Feature SW override value for ADC Buffer Ping Pong select"]
    #[inline(always)]
    #[must_use]
    pub fn adcbufpipoovrval(&mut self) -> AdcbufpipoovrvalW<Adcbufcfg1Spec> {
        AdcbufpipoovrvalW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
This needs to be programmed to 0x1 in AR16xx 0 --> Interleaved, 1 --> Non-interleaved"]
    #[inline(always)]
    #[must_use]
    pub fn adcbufwritemode(&mut self) -> AdcbufwritemodeW<Adcbufcfg1Spec> {
        AdcbufwritemodeW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Continuous mode enable for ADC Buffer. This is set when a fixed number of samples have to be stored in Ping/Pong and not depend on Chirp time-lines (Eg: Analog Lab characterization to stream out continuous data from DFE). Continous mode is expected to be only used for CZ and ADC Buffer Testpattern mode"]
    #[inline(always)]
    #[must_use]
    pub fn adcbufcontmodeen(&mut self) -> AdcbufcontmodeenW<Adcbufcfg1Spec> {
        AdcbufcontmodeenW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Start Pulse for Continuous mode. The data capture will start from Address 0 once this register is set. All the other configurations like Enable, Sample Count are expected to be programmed before this pulse. Continous mode is expected to be only used for CZ and ADC Buffer Testpattern mode : Its a wspecial access type, write to this field will generate a pulse"]
    #[inline(always)]
    #[must_use]
    pub fn adcbufcontstrtpl(&mut self) -> AdcbufcontstrtplW<Adcbufcfg1Spec> {
        AdcbufcontstrtplW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Stop Pulse for Continuous mode. The data capture will stop once this register is set. Continous mode is expected to be only used for CZ and ADC Buffer Testpattern mode : Its a wspecial access type, write to this field will generate a pulse"]
    #[inline(always)]
    #[must_use]
    pub fn adcbufcontstoppl(&mut self) -> AdcbufcontstopplW<Adcbufcfg1Spec> {
        AdcbufcontstopplW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
TI Internal Feature Ping-pong select value from ADC Buffer Packing logic. Even in SW override mode, this register will indicate the ping-pong select signal generated from the ADC Buffer Packing logic and not the override value."]
    #[inline(always)]
    #[must_use]
    pub fn adcbufpiposel(&mut self) -> AdcbufpiposelW<Adcbufcfg1Spec> {
        AdcbufpiposelW::new(self, 16)
    }
}
#[doc = "ADCBUFCFG1\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbufcfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbufcfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcbufcfg1Spec;
impl crate::RegisterSpec for Adcbufcfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcbufcfg1::R`](R) reader structure"]
impl crate::Readable for Adcbufcfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`adcbufcfg1::W`](W) writer structure"]
impl crate::Writable for Adcbufcfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCBUFCFG1 to value 0"]
impl crate::Resettable for Adcbufcfg1Spec {
    const RESET_VALUE: u32 = 0;
}
