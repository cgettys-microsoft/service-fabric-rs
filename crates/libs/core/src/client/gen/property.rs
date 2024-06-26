pub struct IFabricPropertyManagementClient2Wrap { com : :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricPropertyManagementClient2 }
impl Default for IFabricPropertyManagementClient2Wrap {
    fn default() -> Self {
        Self::new()
    }
}
impl IFabricPropertyManagementClient2Wrap {
    pub fn new() -> IFabricPropertyManagementClient2Wrap {
        IFabricPropertyManagementClient2Wrap { com : crate :: sync :: CreateLocalClient :: < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricPropertyManagementClient2 > () , }
    }
    pub fn from_com(
        com : :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricPropertyManagementClient2,
    ) -> IFabricPropertyManagementClient2Wrap {
        IFabricPropertyManagementClient2Wrap { com }
    }
    pub fn CreateName(
        &self,
        name: ::mssf_com::Microsoft::ServiceFabric::FABRIC_URI,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndCreateName(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginCreateName(name, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn DeleteName(
        &self,
        name: ::mssf_com::Microsoft::ServiceFabric::FABRIC_URI,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndDeleteName(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginDeleteName(name, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn DeleteProperty(
        &self,
        name: ::mssf_com::Microsoft::ServiceFabric::FABRIC_URI,
        propertyName: ::windows_core::PCWSTR,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndDeleteProperty(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginDeleteProperty(name, propertyName, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn EnumerateProperties (& self , name : :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_URI , includeValues : windows :: Win32 :: Foundation :: BOOLEAN , previousResult : & :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricPropertyEnumerationResult , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricPropertyEnumerationResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndEnumerateProperties(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginEnumerateProperties(
                name,
                includeValues,
                previousResult,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn EnumerateSubNames (& self , name : :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_URI , previousResult : & :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricNameEnumerationResult , recursive : windows :: Win32 :: Foundation :: BOOLEAN , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricNameEnumerationResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndEnumerateSubNames(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginEnumerateSubNames(
                name,
                previousResult,
                recursive,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetProperty (& self , name : :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_URI , propertyName : :: windows_core :: PCWSTR , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricPropertyValueResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetProperty(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetProperty(name, propertyName, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }    pub fn GetPropertyMetadata (& self , name : :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_URI , propertyName : :: windows_core :: PCWSTR , timeoutMilliseconds : u32) -> crate :: sync :: FabricReceiver < :: windows_core :: Result < :: mssf_com :: Microsoft :: ServiceFabric :: FabricCommon :: FabricClient :: IFabricPropertyMetadataResult >>{
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndGetPropertyMetadata(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginGetPropertyMetadata(name, propertyName, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn NameExists(
        &self,
        name: ::mssf_com::Microsoft::ServiceFabric::FABRIC_URI,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<u8>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndNameExists(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginNameExists(name, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn PutCustomPropertyOperation(
        &self,
        name: ::mssf_com::Microsoft::ServiceFabric::FABRIC_URI,
        propertyOperation : & :: mssf_com :: Microsoft :: ServiceFabric :: FABRIC_PUT_CUSTOM_PROPERTY_OPERATION,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndPutCustomPropertyOperation(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginPutCustomPropertyOperation(
                name,
                propertyOperation,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn PutPropertyDouble(
        &self,
        name: ::mssf_com::Microsoft::ServiceFabric::FABRIC_URI,
        propertyName: ::windows_core::PCWSTR,
        data: f64,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndPutPropertyDouble(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginPutPropertyDouble(
                name,
                propertyName,
                data,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn PutPropertyGuid(
        &self,
        name: ::mssf_com::Microsoft::ServiceFabric::FABRIC_URI,
        propertyName: ::windows_core::PCWSTR,
        data: &::windows_core::GUID,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndPutPropertyGuid(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginPutPropertyGuid(name, propertyName, data, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn PutPropertyInt64(
        &self,
        name: ::mssf_com::Microsoft::ServiceFabric::FABRIC_URI,
        propertyName: ::windows_core::PCWSTR,
        data: i64,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndPutPropertyInt64(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com
                .BeginPutPropertyInt64(name, propertyName, data, timeoutMilliseconds, &callback)
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
    pub fn PutPropertyWString(
        &self,
        name: ::mssf_com::Microsoft::ServiceFabric::FABRIC_URI,
        propertyName: ::windows_core::PCWSTR,
        data: ::windows_core::PCWSTR,
        timeoutMilliseconds: u32,
    ) -> crate::sync::FabricReceiver<::windows_core::Result<()>> {
        let (tx, rx) = crate::sync::oneshot_channel();
        let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
            let res = unsafe { self.com.EndPutPropertyWString(ctx) };
            tx.send(res);
        });
        let ctx = unsafe {
            self.com.BeginPutPropertyWString(
                name,
                propertyName,
                data,
                timeoutMilliseconds,
                &callback,
            )
        };
        if ctx.is_err() {
            let (tx2, rx2) = crate::sync::oneshot_channel();
            tx2.send(Err(ctx.err().unwrap()));
            rx2
        } else {
            rx
        }
    }
}
