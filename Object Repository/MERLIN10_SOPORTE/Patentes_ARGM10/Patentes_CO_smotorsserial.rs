<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Patentes_CO_smotorsserial</name>
   <tag></tag>
   <elementGuidId>c38cbfd4-32d9-467c-a33f-3e6a8fdeca1c</elementGuidId>
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
            &lt;clientAccessCode>aea243aba41084aa098b3a70eeb63ddf&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapter> &lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;xVehicle>
               &lt;!--Optional:-->
               &lt;domain>AA004FG&lt;/domain>
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

WS.verifyElementText(response, 'infoDomainServiceResponse.return.nVehicle.domain', 'AA004FG')
WS.verifyElementText(response, 'infoDomainServiceResponse.return.nVehicle.vehicleBrand', 'TOYOTA')
WS.verifyElementText(response, 'infoDomainServiceResponse.return.nVehicle.model', 'ETIOS X 1.5 M/T')
WS.verifyElementText(response, 'infoDomainServiceResponse.return.nVehicle.motor', '')
WS.verifyElementText(response, 'infoDomainServiceResponse.return.nVehicle.chassis', '')
WS.verifyElementText(response, 'infoDomainServiceResponse.return.nVehicle.fabricationPlace', '')
WS.verifyElementText(response, 'infoDomainServiceResponse.return.nVehicle.year', '2016')</verificationScript>
   <wsdlAddress>${Patentes_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
