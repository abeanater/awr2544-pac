#[doc = "Register `DMA2CTRL` reader"]
pub type R = crate::R<Dma2ctrlSpec>;
#[doc = "Register `DMA2CTRL` writer"]
pub type W = crate::W<Dma2ctrlSpec>;
#[doc = "Field `COUNT` reader - 5:0\\]
Actual number of remaining DMA transfer COUNTx\\[5:0\\]
is a read-only bit field. It comprises the actual number of DMA transfers that remain, until the DMA channel is disabled if ONESHOTx is set."]
pub type CountR = crate::FieldReader;
#[doc = "Field `COUNT` writer - 5:0\\]
Actual number of remaining DMA transfer COUNTx\\[5:0\\]
is a read-only bit field. It comprises the actual number of DMA transfers that remain, until the DMA channel is disabled if ONESHOTx is set."]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `COUNTBIT17` reader - 6:6\\]
The 17th bit of COUNT field of DMAxCOUNT register. This bit is useful only when ICOUNTx in DMAxCOUNT register is programmed to be 0xFFFF. During all other values, this bit remains to be ΓÇÿ0ΓÇÖ."]
pub type Countbit17R = crate::BitReader;
#[doc = "Field `COUNTBIT17` writer - 6:6\\]
The 17th bit of COUNT field of DMAxCOUNT register. This bit is useful only when ICOUNTx in DMAxCOUNT register is programmed to be 0xFFFF. During all other values, this bit remains to be ΓÇÿ0ΓÇÖ."]
pub type Countbit17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFID7` reader - 7:7\\]
Extended bit of BUFIDx field when Extended Buffer feature is implemented. This bit represents the 8th bit of BUFID field such that any buffers between 127-255 can be configured as DMA capable buffers"]
pub type Bufid7R = crate::BitReader;
#[doc = "Field `BUFID7` writer - 7:7\\]
Extended bit of BUFIDx field when Extended Buffer feature is implemented. This bit represents the 8th bit of BUFID field such that any buffers between 127-255 can be configured as DMA capable buffers"]
pub type Bufid7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICOUNT` reader - 12:8\\]
Initial Count of DMA transfers ICOUNTx\\[4:0\\]
is used to preset the transfer counter COUNTx\\[4:0\\]. Every time COUNTx\\[4:0\\]
hits zero it is reloaded with ICOUNTx\\[4:0\\]. The real number of transfer equals ICOUNTx\\[4:0\\]
plus one. If ONESHOTx is set, ICOUNTx\\[4:0\\]
defines the number of DMA transfers that are performed before the MibSPI automatically disables the DMA channels. If NOBRKx is set, ICOUNTx\\[4:0\\]
defines the number of DMA transfers that are performed in one sequence without a transfer from any other buffer"]
pub type IcountR = crate::FieldReader;
#[doc = "Field `ICOUNT` writer - 12:8\\]
Initial Count of DMA transfers ICOUNTx\\[4:0\\]
is used to preset the transfer counter COUNTx\\[4:0\\]. Every time COUNTx\\[4:0\\]
hits zero it is reloaded with ICOUNTx\\[4:0\\]. The real number of transfer equals ICOUNTx\\[4:0\\]
plus one. If ONESHOTx is set, ICOUNTx\\[4:0\\]
defines the number of DMA transfers that are performed before the MibSPI automatically disables the DMA channels. If NOBRKx is set, ICOUNTx\\[4:0\\]
defines the number of DMA transfers that are performed in one sequence without a transfer from any other buffer"]
pub type IcountW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NOBRK` reader - 13:13\\]
Non-interleaved DMA block transfer(Master mode only). 1 =NOBRKx ensures that ICOUNTx+1 data transfers are performed from the buffer referenced by BUFIDx without a data transfer from any other buffer. The sequencer remains at the DMA buffer until ICOUNTx+1 transfers have been processed. E.g: this can be used to generate a burst transfer to one device without disabling the chip select signal in-between (the concerned buffer has to be configured with CSHOLD=1). Another example would be to have a defined block data transfer in slave mode, synchronous to the master SPI. Triggering of higher priority transfer groups or enabling of higher priority DMA channels will not interrupt NOBRK block transfer. 0 =The DMA transfers through the buffer referenced by BUFIDx are interleaved by data transfers from other active buffers or transfer groups. Every time the sequencer checks the DMA buffer, it performs one transfer and then steps to the next buffer"]
pub type NobrkR = crate::BitReader;
#[doc = "Field `NOBRK` writer - 13:13\\]
Non-interleaved DMA block transfer(Master mode only). 1 =NOBRKx ensures that ICOUNTx+1 data transfers are performed from the buffer referenced by BUFIDx without a data transfer from any other buffer. The sequencer remains at the DMA buffer until ICOUNTx+1 transfers have been processed. E.g: this can be used to generate a burst transfer to one device without disabling the chip select signal in-between (the concerned buffer has to be configured with CSHOLD=1). Another example would be to have a defined block data transfer in slave mode, synchronous to the master SPI. Triggering of higher priority transfer groups or enabling of higher priority DMA channels will not interrupt NOBRK block transfer. 0 =The DMA transfers through the buffer referenced by BUFIDx are interleaved by data transfers from other active buffers or transfer groups. Every time the sequencer checks the DMA buffer, it performs one transfer and then steps to the next buffer"]
pub type NobrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAENA` reader - 14:14\\]
Transmit data DMA channel enable. 1 =The physical DMA Request line for the transmit path is enabled. The first DMA request pulse is generated right after setting TXDMAENAx to load the first transmit data. The concerned buffer should be configured in the mode ΓÇ£skip until TXFULL is setΓÇ¥ or ΓÇ£suspend to wait until TXFULL is setΓÇ¥ in order to ensure synchronization between DMA controller and MibSPI sequencer. 0 =No DMA request upon new transmit data"]
pub type TxdmaenaR = crate::BitReader;
#[doc = "Field `TXDMAENA` writer - 14:14\\]
Transmit data DMA channel enable. 1 =The physical DMA Request line for the transmit path is enabled. The first DMA request pulse is generated right after setting TXDMAENAx to load the first transmit data. The concerned buffer should be configured in the mode ΓÇ£skip until TXFULL is setΓÇ¥ or ΓÇ£suspend to wait until TXFULL is setΓÇ¥ in order to ensure synchronization between DMA controller and MibSPI sequencer. 0 =No DMA request upon new transmit data"]
pub type TxdmaenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMAENA` reader - 15:15\\]
Receive data DMA channel enable. 1 =The physical DMA Request line for the receive path is enabled. The first DMA request pulse is generated after the first transfer from the referenced buffer (BUFIDx) is finished. The concerned buffer should be configured in the mode ΓÇ£skip until RXEMPTY is setΓÇ¥ or ΓÇ£suspend to wait until RXEMPTY is setΓÇ¥ in order to ensure synchronization between DMA controller and MibSPI sequencer. 0 =No DMA request upon new receive data."]
pub type RxdmaenaR = crate::BitReader;
#[doc = "Field `RXDMAENA` writer - 15:15\\]
Receive data DMA channel enable. 1 =The physical DMA Request line for the receive path is enabled. The first DMA request pulse is generated after the first transfer from the referenced buffer (BUFIDx) is finished. The concerned buffer should be configured in the mode ΓÇ£skip until RXEMPTY is setΓÇ¥ or ΓÇ£suspend to wait until RXEMPTY is setΓÇ¥ in order to ensure synchronization between DMA controller and MibSPI sequencer. 0 =No DMA request upon new receive data."]
pub type RxdmaenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMA_MAP` reader - 19:16\\]
Transmit data DMA channel Each MibSPI DMA channel can be linked to two physical DMA Request lines of the DMA controller. ne request line for receive data and the other for request line for transmit data. TXDMA_MAPx\\[3:0\\]
defines the number of the physical DMA Request line that is connected to the transmit path of the MibSPI DMA channel. If RXDMAENAx and TXDMAENAx are both set then TXDMA_MAPx\\[3:0\\]
shall differ from RXDMA_MAPx\\[3:0\\]
and shall differ from any other used physical DMA Request line. Otherwise unexpected interference may occur."]
pub type TxdmaMapR = crate::FieldReader;
#[doc = "Field `TXDMA_MAP` writer - 19:16\\]
Transmit data DMA channel Each MibSPI DMA channel can be linked to two physical DMA Request lines of the DMA controller. ne request line for receive data and the other for request line for transmit data. TXDMA_MAPx\\[3:0\\]
defines the number of the physical DMA Request line that is connected to the transmit path of the MibSPI DMA channel. If RXDMAENAx and TXDMAENAx are both set then TXDMA_MAPx\\[3:0\\]
shall differ from RXDMA_MAPx\\[3:0\\]
and shall differ from any other used physical DMA Request line. Otherwise unexpected interference may occur."]
pub type TxdmaMapW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RXDMA_MAP` reader - 23:20\\]
Receive data DMA Request Map Each MibSPI DMA channel can be linked to two physical DMA Request lines of the DMA controller. One request line for receive data and the other for request line for transmit data. RXDMA_MAPx\\[3:0\\]
defines the number of the physical DMA Request line that is connected to the receive path of the MibSPI DMA channel. If RXDMAENAx and TXDMAENAx are both set to ΓÇÿ1ΓÇÖ, then RXDMA_MAPx\\[3:0\\]
shall differ from TXDMA_MAPx\\[3:0\\]
and shall differ from any other used physical DMA Request line. Otherwise unexpected interference may occur."]
pub type RxdmaMapR = crate::FieldReader;
#[doc = "Field `RXDMA_MAP` writer - 23:20\\]
Receive data DMA Request Map Each MibSPI DMA channel can be linked to two physical DMA Request lines of the DMA controller. One request line for receive data and the other for request line for transmit data. RXDMA_MAPx\\[3:0\\]
defines the number of the physical DMA Request line that is connected to the receive path of the MibSPI DMA channel. If RXDMAENAx and TXDMAENAx are both set to ΓÇÿ1ΓÇÖ, then RXDMA_MAPx\\[3:0\\]
shall differ from TXDMA_MAPx\\[3:0\\]
and shall differ from any other used physical DMA Request line. Otherwise unexpected interference may occur."]
pub type RxdmaMapW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BUFID` reader - 30:24\\]
Buffer utilized for DMA transfer. BUFIDx defines the buffer that is utilized for the DMA transfer. In order to synchronize the transfer with the DMA controller with the NOBRK condition the ΓÇ£suspend to wait until...ΓÇ¥ modes must be used (for more details refer to Section 8.53.1)."]
pub type BufidR = crate::FieldReader;
#[doc = "Field `BUFID` writer - 30:24\\]
Buffer utilized for DMA transfer. BUFIDx defines the buffer that is utilized for the DMA transfer. In order to synchronize the transfer with the DMA controller with the NOBRK condition the ΓÇ£suspend to wait until...ΓÇ¥ modes must be used (for more details refer to Section 8.53.1)."]
pub type BufidW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ONESHOT` reader - 31:31\\]
Auto-disable of DMA channel after ICOUNT+1 transfers. 1 =ONESHOTx allows a block transfer of defined length (ICOUNTx+1) mainly controlled by the MibSPI and not by the DMA controller. After ICOUNTx +1 transfers the enable bits RXDMAENAx and TXDMAENAx are automatically cleared by the MibSPI, hence no more DMA requests are generated. In conjunction with NOBRKx, a burst transfer can be initiated without any other transfer through another buffer. 0 =The length of the block transfer is fully controlled by the DMA controller. The enable bits RXDMAENAx and TXDMAENAx are not modified by the MibSPI."]
pub type OneshotR = crate::BitReader;
#[doc = "Field `ONESHOT` writer - 31:31\\]
Auto-disable of DMA channel after ICOUNT+1 transfers. 1 =ONESHOTx allows a block transfer of defined length (ICOUNTx+1) mainly controlled by the MibSPI and not by the DMA controller. After ICOUNTx +1 transfers the enable bits RXDMAENAx and TXDMAENAx are automatically cleared by the MibSPI, hence no more DMA requests are generated. In conjunction with NOBRKx, a burst transfer can be initiated without any other transfer through another buffer. 0 =The length of the block transfer is fully controlled by the DMA controller. The enable bits RXDMAENAx and TXDMAENAx are not modified by the MibSPI."]
pub type OneshotW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Actual number of remaining DMA transfer COUNTx\\[5:0\\]
is a read-only bit field. It comprises the actual number of DMA transfers that remain, until the DMA channel is disabled if ONESHOTx is set."]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
The 17th bit of COUNT field of DMAxCOUNT register. This bit is useful only when ICOUNTx in DMAxCOUNT register is programmed to be 0xFFFF. During all other values, this bit remains to be ΓÇÿ0ΓÇÖ."]
    #[inline(always)]
    pub fn countbit17(&self) -> Countbit17R {
        Countbit17R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Extended bit of BUFIDx field when Extended Buffer feature is implemented. This bit represents the 8th bit of BUFID field such that any buffers between 127-255 can be configured as DMA capable buffers"]
    #[inline(always)]
    pub fn bufid7(&self) -> Bufid7R {
        Bufid7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Initial Count of DMA transfers ICOUNTx\\[4:0\\]
is used to preset the transfer counter COUNTx\\[4:0\\]. Every time COUNTx\\[4:0\\]
hits zero it is reloaded with ICOUNTx\\[4:0\\]. The real number of transfer equals ICOUNTx\\[4:0\\]
plus one. If ONESHOTx is set, ICOUNTx\\[4:0\\]
defines the number of DMA transfers that are performed before the MibSPI automatically disables the DMA channels. If NOBRKx is set, ICOUNTx\\[4:0\\]
defines the number of DMA transfers that are performed in one sequence without a transfer from any other buffer"]
    #[inline(always)]
    pub fn icount(&self) -> IcountR {
        IcountR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Non-interleaved DMA block transfer(Master mode only). 1 =NOBRKx ensures that ICOUNTx+1 data transfers are performed from the buffer referenced by BUFIDx without a data transfer from any other buffer. The sequencer remains at the DMA buffer until ICOUNTx+1 transfers have been processed. E.g: this can be used to generate a burst transfer to one device without disabling the chip select signal in-between (the concerned buffer has to be configured with CSHOLD=1). Another example would be to have a defined block data transfer in slave mode, synchronous to the master SPI. Triggering of higher priority transfer groups or enabling of higher priority DMA channels will not interrupt NOBRK block transfer. 0 =The DMA transfers through the buffer referenced by BUFIDx are interleaved by data transfers from other active buffers or transfer groups. Every time the sequencer checks the DMA buffer, it performs one transfer and then steps to the next buffer"]
    #[inline(always)]
    pub fn nobrk(&self) -> NobrkR {
        NobrkR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Transmit data DMA channel enable. 1 =The physical DMA Request line for the transmit path is enabled. The first DMA request pulse is generated right after setting TXDMAENAx to load the first transmit data. The concerned buffer should be configured in the mode ΓÇ£skip until TXFULL is setΓÇ¥ or ΓÇ£suspend to wait until TXFULL is setΓÇ¥ in order to ensure synchronization between DMA controller and MibSPI sequencer. 0 =No DMA request upon new transmit data"]
    #[inline(always)]
    pub fn txdmaena(&self) -> TxdmaenaR {
        TxdmaenaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Receive data DMA channel enable. 1 =The physical DMA Request line for the receive path is enabled. The first DMA request pulse is generated after the first transfer from the referenced buffer (BUFIDx) is finished. The concerned buffer should be configured in the mode ΓÇ£skip until RXEMPTY is setΓÇ¥ or ΓÇ£suspend to wait until RXEMPTY is setΓÇ¥ in order to ensure synchronization between DMA controller and MibSPI sequencer. 0 =No DMA request upon new receive data."]
    #[inline(always)]
    pub fn rxdmaena(&self) -> RxdmaenaR {
        RxdmaenaR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Transmit data DMA channel Each MibSPI DMA channel can be linked to two physical DMA Request lines of the DMA controller. ne request line for receive data and the other for request line for transmit data. TXDMA_MAPx\\[3:0\\]
defines the number of the physical DMA Request line that is connected to the transmit path of the MibSPI DMA channel. If RXDMAENAx and TXDMAENAx are both set then TXDMA_MAPx\\[3:0\\]
shall differ from RXDMA_MAPx\\[3:0\\]
and shall differ from any other used physical DMA Request line. Otherwise unexpected interference may occur."]
    #[inline(always)]
    pub fn txdma_map(&self) -> TxdmaMapR {
        TxdmaMapR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Receive data DMA Request Map Each MibSPI DMA channel can be linked to two physical DMA Request lines of the DMA controller. One request line for receive data and the other for request line for transmit data. RXDMA_MAPx\\[3:0\\]
defines the number of the physical DMA Request line that is connected to the receive path of the MibSPI DMA channel. If RXDMAENAx and TXDMAENAx are both set to ΓÇÿ1ΓÇÖ, then RXDMA_MAPx\\[3:0\\]
shall differ from TXDMA_MAPx\\[3:0\\]
and shall differ from any other used physical DMA Request line. Otherwise unexpected interference may occur."]
    #[inline(always)]
    pub fn rxdma_map(&self) -> RxdmaMapR {
        RxdmaMapR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Buffer utilized for DMA transfer. BUFIDx defines the buffer that is utilized for the DMA transfer. In order to synchronize the transfer with the DMA controller with the NOBRK condition the ΓÇ£suspend to wait until...ΓÇ¥ modes must be used (for more details refer to Section 8.53.1)."]
    #[inline(always)]
    pub fn bufid(&self) -> BufidR {
        BufidR::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Auto-disable of DMA channel after ICOUNT+1 transfers. 1 =ONESHOTx allows a block transfer of defined length (ICOUNTx+1) mainly controlled by the MibSPI and not by the DMA controller. After ICOUNTx +1 transfers the enable bits RXDMAENAx and TXDMAENAx are automatically cleared by the MibSPI, hence no more DMA requests are generated. In conjunction with NOBRKx, a burst transfer can be initiated without any other transfer through another buffer. 0 =The length of the block transfer is fully controlled by the DMA controller. The enable bits RXDMAENAx and TXDMAENAx are not modified by the MibSPI."]
    #[inline(always)]
    pub fn oneshot(&self) -> OneshotR {
        OneshotR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Actual number of remaining DMA transfer COUNTx\\[5:0\\]
is a read-only bit field. It comprises the actual number of DMA transfers that remain, until the DMA channel is disabled if ONESHOTx is set."]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> CountW<Dma2ctrlSpec> {
        CountW::new(self, 0)
    }
    #[doc = "Bit 6 - 6:6\\]
The 17th bit of COUNT field of DMAxCOUNT register. This bit is useful only when ICOUNTx in DMAxCOUNT register is programmed to be 0xFFFF. During all other values, this bit remains to be ΓÇÿ0ΓÇÖ."]
    #[inline(always)]
    #[must_use]
    pub fn countbit17(&mut self) -> Countbit17W<Dma2ctrlSpec> {
        Countbit17W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Extended bit of BUFIDx field when Extended Buffer feature is implemented. This bit represents the 8th bit of BUFID field such that any buffers between 127-255 can be configured as DMA capable buffers"]
    #[inline(always)]
    #[must_use]
    pub fn bufid7(&mut self) -> Bufid7W<Dma2ctrlSpec> {
        Bufid7W::new(self, 7)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Initial Count of DMA transfers ICOUNTx\\[4:0\\]
is used to preset the transfer counter COUNTx\\[4:0\\]. Every time COUNTx\\[4:0\\]
hits zero it is reloaded with ICOUNTx\\[4:0\\]. The real number of transfer equals ICOUNTx\\[4:0\\]
plus one. If ONESHOTx is set, ICOUNTx\\[4:0\\]
defines the number of DMA transfers that are performed before the MibSPI automatically disables the DMA channels. If NOBRKx is set, ICOUNTx\\[4:0\\]
defines the number of DMA transfers that are performed in one sequence without a transfer from any other buffer"]
    #[inline(always)]
    #[must_use]
    pub fn icount(&mut self) -> IcountW<Dma2ctrlSpec> {
        IcountW::new(self, 8)
    }
    #[doc = "Bit 13 - 13:13\\]
Non-interleaved DMA block transfer(Master mode only). 1 =NOBRKx ensures that ICOUNTx+1 data transfers are performed from the buffer referenced by BUFIDx without a data transfer from any other buffer. The sequencer remains at the DMA buffer until ICOUNTx+1 transfers have been processed. E.g: this can be used to generate a burst transfer to one device without disabling the chip select signal in-between (the concerned buffer has to be configured with CSHOLD=1). Another example would be to have a defined block data transfer in slave mode, synchronous to the master SPI. Triggering of higher priority transfer groups or enabling of higher priority DMA channels will not interrupt NOBRK block transfer. 0 =The DMA transfers through the buffer referenced by BUFIDx are interleaved by data transfers from other active buffers or transfer groups. Every time the sequencer checks the DMA buffer, it performs one transfer and then steps to the next buffer"]
    #[inline(always)]
    #[must_use]
    pub fn nobrk(&mut self) -> NobrkW<Dma2ctrlSpec> {
        NobrkW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Transmit data DMA channel enable. 1 =The physical DMA Request line for the transmit path is enabled. The first DMA request pulse is generated right after setting TXDMAENAx to load the first transmit data. The concerned buffer should be configured in the mode ΓÇ£skip until TXFULL is setΓÇ¥ or ΓÇ£suspend to wait until TXFULL is setΓÇ¥ in order to ensure synchronization between DMA controller and MibSPI sequencer. 0 =No DMA request upon new transmit data"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaena(&mut self) -> TxdmaenaW<Dma2ctrlSpec> {
        TxdmaenaW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Receive data DMA channel enable. 1 =The physical DMA Request line for the receive path is enabled. The first DMA request pulse is generated after the first transfer from the referenced buffer (BUFIDx) is finished. The concerned buffer should be configured in the mode ΓÇ£skip until RXEMPTY is setΓÇ¥ or ΓÇ£suspend to wait until RXEMPTY is setΓÇ¥ in order to ensure synchronization between DMA controller and MibSPI sequencer. 0 =No DMA request upon new receive data."]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaena(&mut self) -> RxdmaenaW<Dma2ctrlSpec> {
        RxdmaenaW::new(self, 15)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Transmit data DMA channel Each MibSPI DMA channel can be linked to two physical DMA Request lines of the DMA controller. ne request line for receive data and the other for request line for transmit data. TXDMA_MAPx\\[3:0\\]
defines the number of the physical DMA Request line that is connected to the transmit path of the MibSPI DMA channel. If RXDMAENAx and TXDMAENAx are both set then TXDMA_MAPx\\[3:0\\]
shall differ from RXDMA_MAPx\\[3:0\\]
and shall differ from any other used physical DMA Request line. Otherwise unexpected interference may occur."]
    #[inline(always)]
    #[must_use]
    pub fn txdma_map(&mut self) -> TxdmaMapW<Dma2ctrlSpec> {
        TxdmaMapW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Receive data DMA Request Map Each MibSPI DMA channel can be linked to two physical DMA Request lines of the DMA controller. One request line for receive data and the other for request line for transmit data. RXDMA_MAPx\\[3:0\\]
defines the number of the physical DMA Request line that is connected to the receive path of the MibSPI DMA channel. If RXDMAENAx and TXDMAENAx are both set to ΓÇÿ1ΓÇÖ, then RXDMA_MAPx\\[3:0\\]
shall differ from TXDMA_MAPx\\[3:0\\]
and shall differ from any other used physical DMA Request line. Otherwise unexpected interference may occur."]
    #[inline(always)]
    #[must_use]
    pub fn rxdma_map(&mut self) -> RxdmaMapW<Dma2ctrlSpec> {
        RxdmaMapW::new(self, 20)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Buffer utilized for DMA transfer. BUFIDx defines the buffer that is utilized for the DMA transfer. In order to synchronize the transfer with the DMA controller with the NOBRK condition the ΓÇ£suspend to wait until...ΓÇ¥ modes must be used (for more details refer to Section 8.53.1)."]
    #[inline(always)]
    #[must_use]
    pub fn bufid(&mut self) -> BufidW<Dma2ctrlSpec> {
        BufidW::new(self, 24)
    }
    #[doc = "Bit 31 - 31:31\\]
Auto-disable of DMA channel after ICOUNT+1 transfers. 1 =ONESHOTx allows a block transfer of defined length (ICOUNTx+1) mainly controlled by the MibSPI and not by the DMA controller. After ICOUNTx +1 transfers the enable bits RXDMAENAx and TXDMAENAx are automatically cleared by the MibSPI, hence no more DMA requests are generated. In conjunction with NOBRKx, a burst transfer can be initiated without any other transfer through another buffer. 0 =The length of the block transfer is fully controlled by the DMA controller. The enable bits RXDMAENAx and TXDMAENAx are not modified by the MibSPI."]
    #[inline(always)]
    #[must_use]
    pub fn oneshot(&mut self) -> OneshotW<Dma2ctrlSpec> {
        OneshotW::new(self, 31)
    }
}
#[doc = "MibSPI DMA Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma2ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma2ctrlSpec;
impl crate::RegisterSpec for Dma2ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma2ctrl::R`](R) reader structure"]
impl crate::Readable for Dma2ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dma2ctrl::W`](W) writer structure"]
impl crate::Writable for Dma2ctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA2CTRL to value 0"]
impl crate::Resettable for Dma2ctrlSpec {
    const RESET_VALUE: u32 = 0;
}
