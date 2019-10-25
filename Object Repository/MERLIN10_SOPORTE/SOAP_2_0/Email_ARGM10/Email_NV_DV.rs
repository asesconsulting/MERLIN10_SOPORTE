<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Email_NV_DV</name>
   <tag></tag>
   <elementGuidId>d2ad244d-6a59-43df-a33e-6b7d4bb50a45</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap.emailonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:emailNormalize>
         &lt;!--Optional:-->
         &lt;emailNormalize>
            &lt;!--Optional:-->
            &lt;clientAccessCode>aea243aba41084aa098b3a70eeb63ddf&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapter> &lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;xEmail>
               &lt;!--Optional:-->
               &lt;email>lvequis@profertil.com.ar &lt;/email>
            &lt;/xEmail>
         &lt;/emailNormalize>
      &lt;/soap:emailNormalize>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>emailNormalize</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Email_ARG2</defaultValue>
      <description></description>
      <id>8698f1dd-5259-4223-b735-fdf097367c4d</id>
      <masked>false</masked>
      <name>Email_ARG2</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyElementText(response, 'emailNormalizeResponse.return.status', 'NV')
WS.verifyElementText(response, 'emailNormalizeResponse.return.statusReason', 'DV')
WS.verifyElementText(response, 'emailNormalizeResponse.return.nEmail.email', 'lvequis@profertil.com.ar')
WS.verifyElementText(response, 'emailNormalizeResponse.return.nEmail.type', 'ND')</verificationScript>
   <wsdlAddress>${Email_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
