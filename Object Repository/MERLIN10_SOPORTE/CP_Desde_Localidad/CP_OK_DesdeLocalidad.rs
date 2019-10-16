<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CP_OK_DesdeLocalidad</name>
   <tag></tag>
   <elementGuidId>8c8b4ec6-ba19-4b13-ab3d-ad9daaa2cd08</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:get=&quot;http://getpcfromlevel4.soap2.addressonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;get:getpcfromlevel4query>
         &lt;!--Optional:-->
         &lt;getpcfromlevel4>
            &lt;!--Optional:-->
            &lt;clientAccessCode>aea243aba41084aa098b3a70eeb63ddf&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapter> &lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;xCity>
               &lt;!--Optional:-->
               &lt;level1> &lt;/level1>
               &lt;!--Optional:-->
               &lt;level2>Buenos aires&lt;/level2>
               &lt;!--Optional:-->
               &lt;level3> &lt;/level3>
               &lt;!--Optional:-->
               &lt;level4>DON TORCUATO&lt;/level4>
               &lt;!--Optional:-->
               &lt;additionalData> &lt;/additionalData>
            &lt;/xCity>
         &lt;/getpcfromlevel4>
      &lt;/get:getpcfromlevel4query>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>getpcfromlevel4query</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.CP_Localidad_ARG2</defaultValue>
      <description></description>
      <id>c922b64d-897a-482b-a089-fc0c1cdfbd49</id>
      <masked>false</masked>
      <name>CP_Localidad_ARG2</name>
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


WS.verifyElementText(response, 'getpcfromlevel4queryResponse.return.status', 'EN')
WS.verifyElementText(response, 'getpcfromlevel4queryResponse.return.statusReason', 'SM')
WS.verifyElementText(response, 'getpcfromlevel4queryResponse.return.totalRecords', '1')
WS.verifyElementText(response, 'getpcfromlevel4queryResponse.return.mSuggest.level1', 'AR')
WS.verifyElementText(response, 'getpcfromlevel4queryResponse.return.mSuggest.level2', 'BUENOS AIRES')
WS.verifyElementText(response, 'getpcfromlevel4queryResponse.return.mSuggest.level3', 'TIGRE')
WS.verifyElementText(response, 'getpcfromlevel4queryResponse.return.mSuggest.level4', 'DON TORCUATO')
WS.verifyElementText(response, 'getpcfromlevel4queryResponse.return.mSuggest.postalCode', '1611')</verificationScript>
   <wsdlAddress>${CP_Localidad_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
