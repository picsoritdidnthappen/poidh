import {
  useDynamicContext,
  DynamicConnectButton,
} from '@dynamic-labs/sdk-react-core'



const walletConnection =async () => {
  console.log("wallet connection")
}






const ConnectWallet = () => {
  
  const {
     isAuthenticated ,
     walletConnector,
     handleLogOut,

    } 
  = useDynamicContext();

  // const handleConnect = async () => {
  //   if (walletConnector?.supportsNetworkSwitching()) {
  //     await walletConnector?.switchNetwork({ networkChainId: 137 });
  //     console.log("Success! Network switched");
  //   }
  // };

  // console.log('isAuthenticated: ' + isAuthenticated )


  return (
    <div className='flex flex-col'>
      

      {!isAuthenticated ?
      <div>

      <DynamicConnectButton>
        
        <button 
        className=' bg-gradient-to-t  from-[#D8393A] to-[#D8393A] flex items-center gap-x-2 border-[#F15E5F] border rounded-md px-2 py-2 '>
        connect
        <svg width="16" height="17" viewBox="0 0 16 17" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M8.00016 2.2795C5.78015 2.2795 3.84088 3.48484 2.80244 5.27998C2.61808 5.59869 2.21026 5.7076 1.89155 5.52324C1.57285 5.33888 1.46394 4.93106 1.6483 4.61235C2.91526 2.42215 5.28495 0.946167 8.00016 0.946167C12.0502 0.946167 15.3335 4.22941 15.3335 8.2795C15.3335 12.3296 12.0502 15.6128 8.00016 15.6128C5.28495 15.6128 2.91526 14.1368 1.6483 11.9466C1.46394 11.6279 1.57285 11.2201 1.89155 11.0358C2.21026 10.8514 2.61808 10.9603 2.80244 11.279C3.84088 13.0742 5.78015 14.2795 8.00016 14.2795C11.3139 14.2795 14.0002 11.5932 14.0002 8.2795C14.0002 4.96579 11.3139 2.2795 8.00016 2.2795Z" fill="white"/>
        <path d="M7.52876 5.14143C7.78911 4.88108 8.21122 4.88108 8.47157 5.14143L11.1382 7.8081C11.3986 8.06845 11.3986 8.49055 11.1382 8.7509L8.47157 11.4176C8.21122 11.6779 7.78911 11.6779 7.52876 11.4176C7.26841 11.1572 7.26841 10.7351 7.52876 10.4748L9.05735 8.94617H2.00016C1.63197 8.94617 1.3335 8.64769 1.3335 8.2795C1.3335 7.91131 1.63197 7.61283 2.00016 7.61283H9.05735L7.52876 6.08424C7.26841 5.82389 7.26841 5.40178 7.52876 5.14143Z" fill="white"/>
        </svg>
        </button>
        
      </DynamicConnectButton>  
      
      </div>

       :
       <button 
       className=' bg-gradient-to-t from-black to-[#7E7E7E] flex items-center gap-x-2 border-[#A6A6A6] border rounded-md px-2 py-2 '
       onClick={() => handleLogOut()}
       >
       disconnect
       <svg className="translate-y-[1px]" width="16" height="17" viewBox="0 0 16 17" fill="none" xmlns="http://www.w3.org/2000/svg">  
        <path d="M0.666748 8.27958C0.666748 4.62162 3.52853 1.61292 7.11119 1.61292C8.28897 1.61292 9.39374 1.94033 10.3434 2.51079C10.659 2.70039 10.7612 3.10995 10.5716 3.42557C10.382 3.74119 9.9724 3.84335 9.65678 3.65376C8.90675 3.2032 8.03819 2.94625 7.11119 2.94625C4.31189 2.94625 2.00008 5.31013 2.00008 8.27958C2.00008 11.249 4.31189 13.6129 7.11119 13.6129C8.03819 13.6129 8.90675 13.356 9.65678 12.9054C9.9724 12.7158 10.382 12.818 10.5716 13.1336C10.7612 13.4492 10.659 13.8588 10.3434 14.0484C9.39374 14.6188 8.28897 14.9462 7.11119 14.9462C3.52853 14.9462 0.666748 11.9375 0.666748 8.27958Z" fill="white"/>
        <path d="M11.5287 5.14151C11.789 4.88116 12.2111 4.88116 12.4715 5.14151L15.1382 7.80818C15.3985 8.06853 15.3985 8.49064 15.1382 8.75099L12.4715 11.4177C12.2111 11.678 11.789 11.678 11.5287 11.4177C11.2683 11.1573 11.2683 10.7352 11.5287 10.4748L13.0573 8.94625H6.00008C5.63189 8.94625 5.33341 8.64777 5.33341 8.27958C5.33341 7.91139 5.63189 7.61292 6.00008 7.61292H13.0573L11.5287 6.08432C11.2683 5.82397 11.2683 5.40186 11.5287 5.14151Z" fill="white"/>
      </svg>
     </button>
       
       }

     


     
      
    </div>
  );
};

export default ConnectWallet;