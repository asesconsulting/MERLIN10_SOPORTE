<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Patentes_CO_FedePat</name>
   <tag></tag>
   <elementGuidId>cc0a702b-4c8d-4734-9fd2-c098e3c81946</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap2.vehicleonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:infoDomainService>
         &lt;!--Optional:-->
         &lt;vehicleNormalize>
            &lt;!--Optional:-->
            &lt;clientAccessCode>ea86ad3f0ed110b6961d1b883688534c&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapter> &lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;xVehicle>
               &lt;!--Optional:-->
               &lt;domain>AAE375&lt;/domain>
               &lt;!--Optional:-->
               &lt;motor> &lt;/motor>
               &lt;!--Optional:-->
               &lt;chassis> &lt;/chassis>
            &lt;/xVehicle>
         &lt;/vehicleNormalize>
      &lt;/soap:infoDomainService>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>infoDomainService</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Patentes_ARG2</defaultValue>
      <description></description>
      <id>255c66f5-9876-4918-9679-359e8eb59ecf</id>
      <masked>false</masked>
      <name>Patentes_ARG2</name>
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


WS.verifyElementText(response, 'infoDomainServiceResponse.return.status', 'CO')
WS.verifyElementText(response, 'infoDomainServiceResponse.return.statusReason', 'SM')
WS.verifyElementText(response, 'infoDomainServiceResponse.return.nVehicle.domain', 'AAE375')
WS.verifyElementText(response, 'infoDomainServiceResponse.return.nVehicle.vehicleBrand', '')
WS.verifyElementText(response, 'infoDomainServiceResponse.return.nVehicle.model', '')
WS.verifyElementText(response, 'infoDomainServiceResponse.return.nVehicle.motor', '159A30388247630')
WS.verifyElementText(response, 'infoDomainServiceResponse.return.nVehicle.chassis', '8AS146000*S5192224')
WS.verifyElementText(response, 'infoDomainServiceResponse.return.nVehicle.fabricationPlace', '')
WS.verifyElementText(response, 'infoDomainServiceResponse.return.nVehicle.year', '1995')</verificationScript>
   <wsdlAddress>${Patentes_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
